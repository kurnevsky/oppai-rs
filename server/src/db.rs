use anyhow::Result;
use derive_more::{From, Into};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub struct Player {
  pub id: Uuid,
  pub nickname: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "provider")]
#[sqlx(rename_all = "lowercase")]
pub enum Provider {
  Google,
  GitLab,
}

pub struct OidcPlayer {
  pub provider: Provider,
  pub subject: String,
  pub email: Option<String>,
  pub email_verified: Option<bool>,
  pub name: Option<String>,
  pub nickname: Option<String>,
  pub preferred_username: Option<String>,
}

impl OidcPlayer {
  fn nickname(&self) -> &str {
    self
      .preferred_username
      .as_deref()
      .or(self.nickname.as_deref())
      .or(self.name.as_deref())
      .unwrap_or(self.subject.as_str())
  }
}

#[derive(From, Into)]
pub struct SqlxDb {
  pool: Pool<Postgres>,
}

impl SqlxDb {
  pub async fn get_or_create_player(&self, oidc_player: OidcPlayer) -> Result<Uuid> {
    let mut tx = self.pool.begin().await?;

    let player_id: Option<(Uuid,)> = sqlx::query_as(
      "
WITH updated AS (
  UPDATE oidc_players
  SET email = $1, email_verified = $2, name = $3, nickname = $4, preferred_username = $5
  WHERE subject = $6 AND provider = $7
  RETURNING player_id
)
SELECT players.id FROM updated
JOIN players ON updated.player_id = players.id
",
    )
    .bind(oidc_player.email.as_ref())
    .bind(oidc_player.email_verified)
    .bind(oidc_player.name.as_ref())
    .bind(oidc_player.nickname.as_ref())
    .bind(oidc_player.preferred_username.as_ref())
    .bind(oidc_player.subject.as_str())
    .bind(oidc_player.provider)
    .fetch_optional(&mut *tx)
    .await?;

    if let Some((player_id,)) = player_id {
      tx.commit().await?;
      return Ok(player_id);
    }

    let player_id = if let Some(email) = oidc_player.email.as_ref() {
      if oidc_player.email_verified == Some(true) {
        let player_id: Option<(Uuid,)> = sqlx::query_as(
          "
SELECT players.id FROM oidc_players
JOIN players ON oidc_players.player_id = players.id
WHERE oidc_players.email = $1
LIMIT 1
",
        )
        .bind(email)
        .fetch_optional(&mut *tx)
        .await?;
        player_id.map(|(player_id,)| player_id)
      } else {
        None
      }
    } else {
      None
    };

    let player_id = if let Some(player_id) = player_id {
      player_id
    } else {
      let (player_id,) = sqlx::query_as(
        "
INSERT INTO players (id, nickname, registration_time)
VALUES (gen_random_uuid(), unique_nickname($1), now())
RETURNING id
",
      )
      .bind(oidc_player.nickname())
      .fetch_one(&mut *tx)
      .await?;
      player_id
    };

    sqlx::query(
      "
INSERT INTO oidc_players (player_id, provider, subject, email, email_verified, \"name\", nickname, preferred_username)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
",
    )
    .bind(player_id)
    .bind(oidc_player.provider)
    .bind(oidc_player.subject)
    .bind(oidc_player.email)
    .bind(oidc_player.email_verified)
    .bind(oidc_player.name)
    .bind(oidc_player.nickname)
    .bind(oidc_player.preferred_username)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(player_id)
  }

  #[cfg(feature = "test")]
  pub async fn get_or_create_test_player(&self, name: String) -> Result<Uuid> {
    let player_id = Uuid::new_v5(&Uuid::default(), name.as_bytes());

    sqlx::query(
      "
INSERT INTO players (id, nickname, registration_time)
VALUES ($1, unique_nickname($2), now())
ON CONFLICT DO NOTHING
",
    )
    .bind(player_id)
    .bind(name)
    .execute(&self.pool)
    .await?;

    Ok(player_id)
  }
}
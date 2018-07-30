use std::sync::Arc;
use rand::XorShiftRng;
use zobrist::Zobrist;
use player::Player;
use field;
use field::Field;

pub fn construct_field(image: &str) -> Field {
  let lines = image.split('\n').map(|line| line.trim_matches(' ')).filter(|line| !line.is_empty()).collect::<Vec<&str>>();
  let height = lines.len() as u32;
  assert!(height > 0);
  let width = lines.first().unwrap().len() as u32;
  assert!(lines.iter().all(|line| line.len() as u32 == width));
  let mut moves = lines.into_iter().enumerate().flat_map(|(y, line)|
    line.chars().enumerate().filter(|&(_, c)| c.to_ascii_lowercase() != c.to_ascii_uppercase()).map(move |(x, c)| (c, x as u32, y as u32))
  ).collect::<Vec<(char, u32, u32)>>();
  moves.sort_by(|&(c1, _, _), &(c2, _, _)| (c1.to_ascii_lowercase(), c1.is_lowercase()).cmp(&(c2.to_ascii_lowercase(), c2.is_lowercase())));
  let mut rng = XorShiftRng::new_unseeded();
  let zobrist = Arc::new(Zobrist::new(field::length(width, height) * 2, &mut rng));
  let mut field = Field::new(width, height, zobrist);
  for (c, x, y) in moves {
    let player = Player::from_bool(c.is_uppercase());
    let pos = field.to_pos(x, y);
    field.put_point(pos, player);
  }
  field
}

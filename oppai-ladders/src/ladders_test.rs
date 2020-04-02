use crate::ladders::ladders;
use oppai_field::construct_field::construct_field;
use oppai_field::field::NonZeroPos;
use oppai_field::player::Player;
use rand::SeedableRng;
use rand_xorshift::XorShiftRng;
use std::sync::atomic::AtomicBool;

const SEED: [u8; 16] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53];

#[test]
fn ladders_escape() {
  let mut rng = XorShiftRng::from_seed(SEED);
  let mut field = construct_field(
    &mut rng,
    "
    .........
    .........
    .........
    .........
    ..aA.....
    .aAAa....
    ..aa.....
    .........
    ",
  );

  let should_stop = AtomicBool::new(false);

  let (pos, score, banned_trajectories) = ladders(&mut field, Player::Red, 0, &should_stop);

  assert_eq!(pos, None);
  assert_eq!(score, 0);
  assert!(!banned_trajectories.is_empty());
}

#[test]
fn ladders_capture_1() {
  let mut rng = XorShiftRng::from_seed(SEED);
  let mut field = construct_field(
    &mut rng,
    "
    .........
    ......a..
    .........
    .........
    ..aA.....
    .aAAa....
    ..aa.....
    .........
    ",
  );

  let should_stop = AtomicBool::new(false);

  let (pos, score, banned_trajectories) = ladders(&mut field, Player::Red, 0, &should_stop);

  assert_eq!(pos, NonZeroPos::new(field.to_pos(3, 3)));
  assert_eq!(score, 3);
  assert!(banned_trajectories.is_empty());
}

#[test]
fn ladders_capture_2() {
  let mut rng = XorShiftRng::from_seed(SEED);
  let mut field = construct_field(
    &mut rng,
    "
    .........
    ......a..
    .........
    .........
    .........
    .aAAa....
    ..aa.....
    .........
    ",
  );

  let should_stop = AtomicBool::new(false);

  let (pos, score, _) = ladders(&mut field, Player::Red, 0, &should_stop);

  assert_eq!(pos, NonZeroPos::new(field.to_pos(2, 4)));
  assert_eq!(score, 2);
}

#[test]
fn ladders_capture_3() {
  let mut rng = XorShiftRng::from_seed(SEED);
  let mut field = construct_field(
    &mut rng,
    "
    .........
    .......a.
    .........
    .........
    .........
    .aAAa....
    ..aa.....
    .........
    ",
  );

  let should_stop = AtomicBool::new(false);

  let (pos, score, _) = ladders(&mut field, Player::Red, 0, &should_stop);

  assert_eq!(pos, NonZeroPos::new(field.to_pos(2, 4)));
  assert_eq!(score, 2);
}

#[test]
fn ladders_capture_4() {
  let mut rng = XorShiftRng::from_seed(SEED);
  let mut field = construct_field(
    &mut rng,
    "
    .........
    .........
    ........a
    ........a
    .........
    .aAAa....
    ..aa.....
    .........
    ",
  );

  let should_stop = AtomicBool::new(false);

  let (pos, score, _) = ladders(&mut field, Player::Red, 0, &should_stop);

  assert_eq!(pos, NonZeroPos::new(field.to_pos(2, 4)));
  assert_eq!(score, 2);
}

#[test]
fn ladders_side_capture_1() {
  let mut rng = XorShiftRng::from_seed(SEED);
  let mut field = construct_field(
    &mut rng,
    "
    ..............
    ...........aa.
    .............a
    .............a
    ...........Aa.
    .......a..AAa.
    ........aaaa..
    .aAAa.........
    ..aa..........
    ..............
    ",
  );

  let should_stop = AtomicBool::new(false);

  let (pos, score, _) = ladders(&mut field, Player::Red, 0, &should_stop);

  assert_eq!(pos, NonZeroPos::new(field.to_pos(2, 6)));
  assert_eq!(score, 2);
}

#[test]
fn ladders_side_capture_2() {
  let mut rng = XorShiftRng::from_seed(SEED);
  let mut field = construct_field(
    &mut rng,
    "
    ..............
    ..............
    ............a.
    ..............
    ..............
    ..............
    .....aa.......
    .aa.aAAA......
    .AA..AAa......
    .aa.aAa.......
    .....a........
    ..............
    ",
  );

  let should_stop = AtomicBool::new(false);

  let (pos, score, _) = ladders(&mut field, Player::Red, 0, &should_stop);

  assert_eq!(pos, NonZeroPos::new(field.to_pos(3, 8)));
  assert_eq!(score, 2);
}

#[test]
fn ladders_fork() {
  let mut rng = XorShiftRng::from_seed(SEED);
  let mut field = construct_field(
    &mut rng,
    "
    ......
    .aa...
    .AA...
    .aaAa.
    ......
    ",
  );

  let should_stop = AtomicBool::new(false);

  let (pos, score, _) = ladders(&mut field, Player::Red, 0, &should_stop);

  assert_eq!(pos, NonZeroPos::new(field.to_pos(3, 2)));
  assert_eq!(score, 1);
}

#[test]
fn ladders_fork_deep() {
  let mut rng = XorShiftRng::from_seed(SEED);
  let mut field = construct_field(
    &mut rng,
    "
    ..a...
    ..A...
    .a....
    .aAAa.
    ..aa..
    ",
  );

  let should_stop = AtomicBool::new(false);

  let (pos, score, _) = ladders(&mut field, Player::Red, 0, &should_stop);

  assert_eq!(pos, NonZeroPos::new(field.to_pos(2, 2)));
  assert_eq!(score, 1);
}

#[test]
fn ladders_fork_stupid() {
  let mut rng = XorShiftRng::from_seed(SEED);
  let mut field = construct_field(
    &mut rng,
    "
    .......
    .aa.aa.
    .AA.AA.
    .aaAaa.
    .......
    ",
  );

  let should_stop = AtomicBool::new(false);

  let (pos, score, _) = ladders(&mut field, Player::Red, 0, &should_stop);

  assert_eq!(pos, None);
  assert_eq!(score, 0);
}

#[test]
fn ladders_stupid() {
  let mut rng = XorShiftRng::from_seed(SEED);
  let mut field = construct_field(
    &mut rng,
    "
    ..........
    ........a.
    ..........
    .Aaa......
    ..AAAa....
    .Aaaa.....
    ..........
    ",
  );

  let should_stop = AtomicBool::new(false);

  let (pos, score, _) = ladders(&mut field, Player::Red, 0, &should_stop);

  assert_eq!(pos, None);
  assert_eq!(score, 0);
}

#[test]
fn ladders_not_viable_1() {
  let mut rng = XorShiftRng::from_seed(SEED);
  let mut field = construct_field(
    &mut rng,
    "
    ..........
    ........a.
    ..........
    .AaaA.....
    .aAAAa....
    .Aaaa.....
    ..........
    ",
  );

  let should_stop = AtomicBool::new(false);

  let (pos, score, _) = ladders(&mut field, Player::Red, 0, &should_stop);
  assert_eq!(pos, None);
  assert_eq!(score, 0);
}

#[test]
fn ladders_viable() {
  let mut rng = XorShiftRng::from_seed(SEED);
  let mut field = construct_field(
    &mut rng,
    "
    .........
    ......a..
    .........
    .........
    ..aa.....
    .aAAA....
    ..aAAa...
    .aAAa....
    ..aa.....
    ",
  );

  let should_stop = AtomicBool::new(false);

  let (pos, score, _) = ladders(&mut field, Player::Red, 0, &should_stop);
  assert_eq!(pos, NonZeroPos::new(field.to_pos(5, 5)));
  assert_eq!(score, 7);
}

#[test]
fn ladders_not_viable_2() {
  let mut rng = XorShiftRng::from_seed(SEED);
  let mut field = construct_field(
    &mut rng,
    "
    ..........
    .......a..
    ..........
    ..........
    ...aa.....
    .AaAAA....
    ...aAAa...
    .AaAAa....
    ...aa.....
    ",
  );

  let should_stop = AtomicBool::new(false);

  let (pos, score, _) = ladders(&mut field, Player::Red, 0, &should_stop);
  assert_eq!(pos, None);
  assert_eq!(score, 0);
}

#[test]
fn ladders_viable_multi() {
  let mut rng = XorShiftRng::from_seed(SEED);
  let mut field = construct_field(
    &mut rng,
    "
    .................
    .......aaaa......
    ......a....aAA...
    .....aA.....aAA..
    .....AA......aAA.
    .....AA.....aAA..
    .....aa....aAA...
    .a.....aaaa......
    .................
    ",
  );

  let should_stop = AtomicBool::new(false);

  let (pos, score, _) = ladders(&mut field, Player::Red, 0, &should_stop);

  assert_eq!(pos, NonZeroPos::new(field.to_pos(4, 4)));
  assert_eq!(score, 5);
}

#[test]
fn ladders_viable_complex() {
  let mut rng = XorShiftRng::from_seed(SEED);
  let mut field = construct_field(
    &mut rng,
    "
    .........
    ...AA.a..
    ..A.a....
    .Aaa.....
    .aA..Aa..
    .aA..aA..
    .aA..A...
    .aAAAaa..
    ..aaa....
    .........
    ",
  );

  let should_stop = AtomicBool::new(false);

  let (_, score, _) = ladders(&mut field, Player::Red, 0, &should_stop);
  // It's possible to capture 8 points here but current method is
  // limited - it doesn't consider ladders after captures.
  assert_eq!(score, 6);
}

#[test]
fn ladders_depth_limit() {
  let mut rng = XorShiftRng::from_seed(SEED);
  let mut field = construct_field(
    &mut rng,
    "
    .............
    .............
    .............
    .............
    .............
    .a.aaa.......
    ...AAA.......
    ..aaaaa......
    .............
    ",
  );

  let should_stop = AtomicBool::new(false);

  let (pos, score, _) = ladders(&mut field, Player::Red, 1, &should_stop);
  assert_eq!(pos, NonZeroPos::new(field.to_pos(6, 6)));
  assert_eq!(score, 3);

  let (pos, score, _) = ladders(&mut field, Player::Red, 2, &should_stop);
  assert_eq!(pos, None);
  assert_eq!(score, 0);
}

#[test]
fn ladders_defending_not_banned() {
  let mut rng = XorShiftRng::from_seed(SEED);
  let mut field = construct_field(
    &mut rng,
    "
    ...........
    ..A.A......
    .AA..A.....
    .Aaa.Aa....
    .aAaaaA....
    .aAAAa.....
    ..aaAAA....
    ....aa.....
    ...........
    ...........
    ...........
    ",
  );

  let should_stop = AtomicBool::new(false);

  let (_, _, banned_trajectories) = ladders(&mut field, Player::Red, 1, &should_stop);
  assert!(banned_trajectories.is_empty());
}
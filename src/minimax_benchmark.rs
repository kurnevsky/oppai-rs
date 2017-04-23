use rand::XorShiftRng;
use test::Bencher;
use player::Player;
use minimax::minimax;
use construct_field::construct_field;

#[bench]
fn find_best_move_1(bencher: &mut Bencher) {
  let field = construct_field(
    "
    .............
    .............
    ...aAa.......
    ..AAa...A....
    ..Aa...A..a..
    ..Aaa.AA.a...
    ..AaaaaAa....
    ..AAa.Aaa....
    ..aaAA.A.....
    .............
    .............
    "
  );
  bencher.iter(|| {
    let mut rng = XorShiftRng::new_unseeded();
    let mut local_field = field.clone();
    minimax(&mut local_field, Player::Black, &mut rng, 6)
  });
}

#[bench]
fn find_best_move_2(bencher: &mut Bencher) {
  let field = construct_field(
    "
    .......
    ...a...
    .......
    ..Aa.A.
    .A...A.
    .AaaaA.
    ..AAAa.
    .....a.
    .......
    "
  );
  bencher.iter(|| {
    let mut rng = XorShiftRng::new_unseeded();
    let mut local_field = field.clone();
    minimax(&mut local_field, Player::Red, &mut rng, 6)
  });
}

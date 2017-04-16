use num_cpus;
use clap::{Arg, ArgGroup, App};

const CONFIG_STR: &'static str = "config";

arg_enum! {
  #[derive(Clone, Copy, PartialEq, Debug)]
  pub enum UcbType {
    Winrate,
    Ucb1,
    Ucb1Tuned
  }
}

arg_enum! {
  #[derive(Clone, Copy, PartialEq, Debug)]
  pub enum UctKomiType {
    None,
    Static,
    Dynamic
  }
}

arg_enum! {
  #[derive(Clone, Copy, PartialEq, Debug)]
  pub enum MinimaxMovesSorting {
    None,
    Random,
    TrajectoriesCount
    // Heuristic
  }
}

arg_enum! {
  #[derive(Clone, Copy, PartialEq, Debug)]
  pub enum Solver {
    Uct,
    NegaScout,
    Heuristic
  }
}

#[derive(Clone, PartialEq, Debug)]
struct Config {
  uct: UctConfig,
  minimax: MinimaxConfig,
  bot: BotConfig
}

#[derive(Clone, PartialEq, Debug)]
struct UctConfig {
  radius: u32,
  ucb_type: UcbType,
  final_ucb_type: UcbType,
  draw_weight: f64,
  uctk: f64,
  when_create_children: usize,
  depth: u32,
  komi_type: UctKomiType,
  red: f64,
  green: f64,
  komi_min_iterations: usize
}

#[derive(Clone, PartialEq, Debug)]
struct MinimaxConfig {
  minimax_moves_sorting: MinimaxMovesSorting
}

#[derive(Clone, PartialEq, Debug)]
struct BotConfig {
  threads_count: usize,
  time_gap: u32,
  solver: Solver
}

const DEFAULT_UCT_CONFIG: UctConfig = UctConfig {
  radius: 3,
  ucb_type: UcbType::Ucb1Tuned,
  final_ucb_type: UcbType::Winrate,
  draw_weight: 0.4,
  uctk: 1.0,
  when_create_children: 2,
  depth: 8,
  komi_type: UctKomiType::Dynamic,
  red: 0.45,
  green: 0.5,
  komi_min_iterations: 3000
};

const DEFAULT_MINIMAX_CONFIG: MinimaxConfig = MinimaxConfig {
  minimax_moves_sorting: MinimaxMovesSorting::Random
};

const DEFAULT_BOT_CONFIG: BotConfig = BotConfig {
  threads_count: 4,
  time_gap: 100,
  solver: Solver::Uct
};

const DEFAULT_CONFIG: Config = Config {
  uct: DEFAULT_UCT_CONFIG,
  minimax: DEFAULT_MINIMAX_CONFIG,
  bot: DEFAULT_BOT_CONFIG
};

static mut CONFIG: Config = DEFAULT_CONFIG;

#[inline]
fn config() -> &'static Config {
  unsafe { &CONFIG }
}

pub fn cli_parse() {
  #[cfg_attr(feature="clippy", allow(zero_ptr))]
  let num_cpus_string = num_cpus::get().to_string();
  let matches = App::new(crate_name!())
    .version(crate_version!())
    .author(crate_authors!("\n"))
    .about(crate_description!())
    .group(ArgGroup::with_name("NegaScout")
           .arg("moves-order"))
    .group(ArgGroup::with_name("NegaScout")
           .args(&[
             "radius",
             "depth",
             "when-create-children",
             "ucb-type",
             "final-ucb-type",
             "uctk",
             "draw-weight",
             "red",
             "green",
             "komi-type",
             "komi-min-iterations"
           ]))
    .arg(Arg::with_name("solver")
         .short("s")
         .long("solver")
         .help("Engine type for position estimation and the best move choosing")
         .takes_value(true)
         .possible_values(&Solver::variants())
         .default_value("Uct"))
    .arg(Arg::with_name("time-gap")
         .short("g")
         .long("time-gap")
         .help("Number of milliseconds that is given to IO plus internal delay")
         .takes_value(true)
         .default_value("100"))
    .arg(Arg::with_name("threads-count")
         .short("t")
         .long("threads-count")
         .help("Number of threads to use. Best performance is achieved by specifying \
                the number of physical CPU cores on the target computer. Will be determined \
                automatically if not specified, but automatic resolution is prone to errors \
                for multithreaded CPU-s")
         .takes_value(true)
         .default_value(&num_cpus_string))
    .arg(Arg::with_name("moves-order")
         .long("moves-order")
         .help("Moves sorting method for NegaScout")
         .takes_value(true)
         .possible_values(&MinimaxMovesSorting::variants())
         .default_value("TrajectoriesCount"))
    .arg(Arg::with_name("radius")
         .long("radius")
         .help("Radius for points that will be considered by UCT search algorithm. \
                The initial points are fixed once the UCT search algorithm starts. After \
                that, only points that are close enough to staring ones are considered. \
                Points that are more distant to any of the starting points are discarted")
         .takes_value(true)
         .default_value("3"))
    .arg(Arg::with_name("depth")
         .long("depth")
         .help("Maximum depth of the UCT tree")
         .takes_value(true)
         .default_value("8"))
    .arg(Arg::with_name("when-create-children")
         .long("when-create-children")
         .help("Child nodes in the UTC tree will be created only after this number of node visits.")
         .takes_value(true)
         .default_value("2"))
    .arg(Arg::with_name("ucb-type")
         .long("ucb-type")
         .help("Formula of the UCT value")
         .takes_value(true)
         .possible_values(&UcbType::variants())
         .default_value("Ucb1Tuned"))
    .arg(Arg::with_name("final-ucb-type")
         .long("final-ucb-type")
         .help("Formula of the UCT value that will be used for best move choosing")
         .takes_value(true)
         .possible_values(&UcbType::variants())
         .default_value("Winrate"))
    .arg(Arg::with_name("uctk")
         .long("uctk")
         .help("UCT constant. Larger values give uniform search. Smaller values give very selective search")
         .takes_value(true)
         .default_value("1.0"))
    .arg(Arg::with_name("draw-weight")
         .long("draw-weight")
         .help("Draw weight for UCT formula. Should be fractional number between 0 \
                (weight of the defeat) and 1 (weight of the win). Smaller values give \
                more aggressive game")
         .takes_value(true)
         .default_value("0.4"))
    .arg(Arg::with_name("red")
         .long("red")
         .help("Red zone for dynamic komi for UCT. Should be fractional number \
                between 0 and 1. Should also be less than green zone")
         .takes_value(true)
         .default_value("0.45"))
    .arg(Arg::with_name("green")
         .long("green")
         .help("Green zone for dynamic komi for UCT. Should be fractional number \
                between 0 and 1. Should also be more than red zone.")
         .takes_value(true)
         .default_value("0.5"))
    .arg(Arg::with_name("komi-type")
         .long("komi-type")
         .help("Type of komi evaluation for UTC during the game")
         .takes_value(true)
         .possible_values(&UctKomiType::variants())
         .default_value("Dynamic"))
    .arg(Arg::with_name("komi-min-iterations")
         .long("komi-min-iterations")
         .help("Dynamic komi for UCT will be updated after this number of iterations")
         .takes_value(true)
         .default_value("3000"))
    .get_matches();
  let uct_config = UctConfig {
    radius: value_t!(matches.value_of("radius"), u32).unwrap_or_else(|e| e.exit()),
    ucb_type: value_t!(matches.value_of("ucb-type"), UcbType).unwrap_or_else(|e| e.exit()),
    final_ucb_type: value_t!(matches.value_of("final-ucb-type"), UcbType).unwrap_or_else(|e| e.exit()),
    draw_weight: value_t!(matches.value_of("draw-weight"), f64).unwrap_or_else(|e| e.exit()),
    uctk: value_t!(matches.value_of("uctk"), f64).unwrap_or_else(|e| e.exit()),
    when_create_children: value_t!(matches.value_of("when-create-children"), usize).unwrap_or_else(|e| e.exit()),
    depth: value_t!(matches.value_of("depth"), u32).unwrap_or_else(|e| e.exit()),
    komi_type: value_t!(matches.value_of("komi-type"), UctKomiType).unwrap_or_else(|e| e.exit()),
    red: value_t!(matches.value_of("red"), f64).unwrap_or_else(|e| e.exit()),
    green: value_t!(matches.value_of("green"), f64).unwrap_or_else(|e| e.exit()),
    komi_min_iterations: value_t!(matches.value_of("komi-min-iterations"), usize).unwrap_or_else(|e| e.exit())
  };
  let minimax_config = MinimaxConfig {
    minimax_moves_sorting: value_t!(matches.value_of("moves-order"), MinimaxMovesSorting).unwrap_or_else(|e| e.exit())
  };
  let bot_config = BotConfig {
    threads_count: value_t!(matches.value_of("threads-count"), usize).unwrap_or_else(|e| e.exit()),
    time_gap: value_t!(matches.value_of("time-gap"), u32).unwrap_or_else(|e| e.exit()),
    solver: value_t!(matches.value_of("solver"), Solver).unwrap_or_else(|e| e.exit())
  };
  let config = Config {
    uct: uct_config,
    minimax: minimax_config,
    bot: bot_config
  };
  unsafe {
    CONFIG = config;
  }
}

#[inline]
pub fn uct_radius() -> u32 {
  config().uct.radius
}

#[inline]
pub fn ucb_type() -> UcbType {
  config().uct.ucb_type
}

#[inline]
pub fn final_ucb_type() -> UcbType {
  config().uct.final_ucb_type
}

#[inline]
pub fn uct_draw_weight() -> f64 {
  config().uct.draw_weight
}

#[inline]
pub fn uctk() -> f64 {
  config().uct.uctk
}

#[inline]
pub fn uct_when_create_children() -> usize {
  config().uct.when_create_children
}

#[inline]
pub fn uct_depth() -> u32 {
  config().uct.depth
}

#[inline]
pub fn threads_count() -> usize {
  config().bot.threads_count
}

#[inline]
pub fn uct_komi_type() -> UctKomiType {
  config().uct.komi_type
}

#[inline]
pub fn uct_red() -> f64 {
  config().uct.red
}

#[inline]
pub fn uct_green() -> f64 {
  config().uct.green
}

#[inline]
pub fn uct_komi_min_iterations() -> usize {
  config().uct.komi_min_iterations
}

#[inline]
pub fn minimax_moves_sorting() -> MinimaxMovesSorting {
  config().minimax.minimax_moves_sorting
}

#[inline]
pub fn time_gap() -> u32 {
  config().bot.time_gap
}

#[inline]
pub fn solver() -> Solver {
  config().bot.solver
}

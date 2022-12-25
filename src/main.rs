use clap::Parser;

mod utils;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_enum, default_value_t=Task::Latest)]
    task: Task,
}

utils::make_runner!(
    1+,
    2+,
    3+,
    4+,
    5+,
    6+,
    7+,
    8+,
    9+,
    10+,
    11+,
    12+,
    13+,
    14+,
    15+,
    16+,
    17+,
    18+,
    19+,
    20+,
    21+,
    22+,
    23+,
    24+,
    25,
);

fn main() {
    run(Args::parse());
}

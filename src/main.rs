use clap::Parser;

mod utils;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_enum)]
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
);

fn main() {
    run(Args::parse());
}

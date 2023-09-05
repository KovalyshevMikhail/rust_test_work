mod computation;

use std::thread;
use std::time::{Instant};
use clap::Parser;
use crate::computation::Computation;

#[derive(Parser, Debug)]
#[command()]
struct Args {
    /// Number of zeros in suffix
    #[arg(short = 'N')]
    number: usize,

    /// Count searches of hashes
    #[arg(short = 'F')]
    found_need: u32,
}

fn main() {
    dotenv::dotenv().ok();
    let max_cpu = thread::available_parallelism().unwrap().get();
    let max_cpu = dotenv::var("MAX_THREADS")
        .unwrap_or(max_cpu.to_string())
        .parse::<usize>().expect("Expect number in environment variable MAX_THREADS");

    let number_range = dotenv::var("NUMBER_RANGE_PER_THREAD")
        .unwrap_or(50_000.to_string())
        .parse::<i32>().expect("Expect number in environment variable NUMBER_RANGE_PER_THREAD");
    println!("Max threads in system: {max_cpu}");

    let args = Args::parse();

    // let need_lost: Arc<Mutex<u32>> = Arc::new(Mutex::new(args.found_need));
    let check_mask = "0".repeat(args.number);

    let start_time = Instant::now();

    let computation = Computation {
        max_cpu,
        number_range,
        check_mask,
        found_need: args.found_need as usize
    };
    let hashes = computation.compute();

    println!("{:#?}", hashes);

    let execution_time = Instant::now() - start_time;
    println!("Execution time: {:?}", execution_time);
}

use clap::Parser;


use std::sync::mpsc::{channel};
use stopwatch::Stopwatch;
mod num_lang;

// TODO: Add startup info, i.e. "Press CTRL + C to stop"
// TODO: Show total session statistics at end.

#[derive(Parser, Debug)]
struct Args {
    /// The maximum candidate to check for primes.
    #[arg(short = 'c', long)]
    max_candidate: Option<u32>,

    /// Limit the number of primes to search for.
    #[arg(short = 'g', long = "goal")]
    prime_goal: Option<u32>,

    /// How often to report major statistics. Set to 0 to disable.
    #[arg(short = 'm', long = "major", default_value_t = 10_000_000)]
    major_interval: u32,

    /// How often to report minor statistics. Set to 0 to disable.
    #[arg(short = 'n', long = "minor", default_value_t = 100_000)]
    minor_interval: u32,

    /// How often to print the table header. Set to 0 to disable.
    #[arg(short = 'i', long = "header", default_value_t = 50)]
    header_interval: u32,
}

fn main() {
    let args = Args::parse();

    let loop_end: u32 = args.max_candidate.unwrap_or(u32::MAX);
    let mut table_header = 0;

    let mut primes: Vec<u32> = vec![3];

    println!("Searching for prime numbers.");
    if args.max_candidate.is_some() {
        println!(
            "Will stop searching after: candidate = {}",
            args.max_candidate.unwrap()
        );
    }
    if args.prime_goal.is_some() {
        println!(
            "Will stop searching after: count = {}",
            args.prime_goal.unwrap()
        );
    }
    if args.prime_goal.is_none() && args.max_candidate.is_none() {
        println!("Will run indefinitely")
    }

    let (tx, rx) = channel();
    ctrlc::set_handler(move || {
        tx.send(()).unwrap();
    })
    .expect("Could not bind Ctrl+C Handler!");

    println!("Press [Ctrl] + [C] to stop.");

    let session_timer = Stopwatch::start_new();
    let mut major_timer = Stopwatch::start_new();
    let mut minor_timer = Stopwatch::start_new();

    if args.minor_interval != 0 {
        header(1);
    }

    for candidate in (5..loop_end).step_by(2) {
        if check_prime(candidate, &primes) {
            primes.push(candidate);
            {
                let len: u32 = (primes.len() + 1) as u32;

                if args.major_interval != 0 && len % args.major_interval == 0 {
                    major_report(args.major_interval, &mut major_timer, candidate);
                    table_header = args.header_interval + 1;
                    minor_timer.restart();
                } else if args.minor_interval != 0 && len % args.minor_interval == 0 {
                    if table_header > args.header_interval {
                        header(args.header_interval);
                        table_header = 1;
                    }
                    minor_report(args.minor_interval, &mut minor_timer, primes.len(), &primes);
                    table_header += 1;
                }
            }
            if primes.len() as u32 >= args.prime_goal.unwrap_or(u32::MAX) {
                break;
            }

            match rx.try_recv() {
                Err(_) => (),
                Ok(_) => { session_report(primes.len(), session_timer.elapsed().as_secs_f32()); }
            }
        }
    }
    session_report(primes.len(), session_timer.elapsed().as_secs_f32());

    fn check_prime(candidate: u32, prime_array: &Vec<u32>) -> bool {
        let sqrt: u32 = (candidate as f32).sqrt() as u32;

        for prime in prime_array {
            if candidate % prime == 0 {
                return false;
            }
            if prime > &sqrt {
                break;
            }
        }
        true
    }
    
    fn minor_report(
        minor_interval: u32,
        minor_timer: &mut Stopwatch,
        last_index: usize,
        prime_array: &[u32],
    ) {
        if minor_interval == 0 {
            return;
        }

        let nth = (last_index + 1) as u32 / minor_interval;
        let prime = prime_array[last_index - 1];
        let time: String;
        if minor_timer.elapsed_ms() < 5000 {
            time = format!("{}ms", minor_timer.elapsed_ms());
        } else {
            time = format!("{:.2}s", minor_timer.elapsed().as_secs_f32())
        }

        println!(" {nth:<5} | {prime:<10} | {time:<5}");
        minor_timer.restart();
    }
    fn major_report(major_interval: u32, major_timer: &mut Stopwatch, last_prime: u32) {
        if major_interval == 0 {
            return;
        }

        let avg = major_interval as f32 / major_timer.elapsed().as_secs_f32();
        let timer_text: String = format!("{:.3}", major_timer.elapsed().as_secs_f32());
        let avg_text: String = format!("{:.3}", avg);
        let last_text: String = num_lang::get_name(major_interval);

        println!("Last {} took {}s", last_text, timer_text);
        println!("Last {}th prime is {}", last_text, last_prime);
        println!("Average speed: {} primes/second", avg_text);
    }

    fn header(interval: u32) {
        if interval == 0 {
            
        } else {
            println!("-------|------------|------");
            println!(" {:5} | {:10} | {:5}", "Nth", "Prime", "Time");
            println!("-------|------------|------");
        }
    }
    fn session_report(len: usize, total_time: f32) {
        println!("\nFinished searching for primes.");
        println!("Took {total_time:.2} seconds.");
        println!(
            "Found {len} prime numbers in {total_time} seconds.");
        let avg = len as f32 / total_time;
        println!("Total average speed: {avg:.2} primes/second");
        std::process::exit(0);
    }
}

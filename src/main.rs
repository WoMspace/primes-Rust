use clap::Parser;
use stopwatch::Stopwatch;
mod num_lang;

#[derive(Parser, Debug)]
struct Args {
    /// The maximum candidate to check for primes.
    #[arg(short = 'c', long)]
    max_candidate: Option<u32>,

    /// The number of primes to search for.
    #[arg(short = 'g', long = "goal", default_value_t = 1_000_000)]
    prime_goal: u32,

    /// How often to report major statistics. Set to 0 to disable.
    #[arg(short = 'm', long = "major", default_value_t = 1_000_000)]
    major_interval: u32,

    /// How often to report minor statistics. Set to 0 to disable.
    #[arg(short = 'n', long = "minor", default_value_t = 10_000)]
    minor_interval: u32,
}

fn main() {
    let args = Args::parse();

    let mut primes: Vec<u32> = vec![3];

    let loop_end: u32 = args.max_candidate.unwrap_or(u32::MAX);

    let mut major_timer = Stopwatch::start_new();
    let mut minor_timer = Stopwatch::start_new();

    for candidate in (5..loop_end).step_by(2) {
        if check_prime(candidate, &primes) {
            primes.push(candidate);
            {
                let len: u32 = (primes.len() + 1) as u32;

                if args.major_interval != 0 && len % args.major_interval == 0 {
                    major_report(args.major_interval, &mut major_timer, candidate);
                    minor_timer.restart();
                } else if args.minor_interval != 0 && len % args.minor_interval == 0 {
                    minor_report(args.minor_interval, &mut minor_timer, primes.len(), &primes);
                }
            }
            if primes.len() as u32 >= args.prime_goal {
                break;
            }
        }
    }

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

        println!("{} | {} | {}", nth, prime, time);
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
}

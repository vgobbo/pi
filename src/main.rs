use std::thread;
use clap::Parser;
use rand::Rng;

type FLOAT = f64;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = num_cpus::get())]
    pub threads: usize,

    #[arg(short, long, default_value_t = 10_000)]
    pub samples: u64,

    #[arg(short, long, default_value_t = 3)]
    pub iterations: usize,
}

fn monte_carlo_fast(samples: u64) -> u64 {
    let mut rng = rand::thread_rng();

    let mut in_circle = 0;

    for _i in 0..samples {
        let x: FLOAT = rng.gen();
        let y: FLOAT = rng.gen();

        let d = x * x + y * y;
        if d <= (1.0 as FLOAT) {
            in_circle += 1;
        }
    }

    in_circle
}

fn monte_carlo_iteration(samples: u64, threads: usize) {
    let mut handles: Vec<_> = Vec::new();

    for _nt in 0..threads {
        let samples = samples.clone();
        handles.push(thread::spawn(move || {
            monte_carlo_fast(samples)
        }));
    }

    let in_circle: u64 = handles.into_iter().map(|handle| handle.join().expect("Failed to join.")).sum();

    let pi: FLOAT = (4.0 as FLOAT) * (in_circle as FLOAT) / ((threads as u64 * samples) as FLOAT);
    println!("{}", pi);
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.iterations {
        monte_carlo_iteration(args.samples, args.threads);
    }
}

use std::thread;
use rand::Rng;

type FLOAT = f64;

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

fn main() {
    let samples = 10000000;
    let nthreads = num_cpus::get() as u64;

    let mut handles: Vec<_> = Vec::new();

    for _nt in 0..nthreads {
        let samples = samples.clone();
        handles.push(thread::spawn(move || {
            monte_carlo_fast(samples)
        }));
    }

    let in_circle: u64 = handles.into_iter().map(|handle| handle.join().expect("Failed to join.")).sum();

    let pi: FLOAT = (4.0 as FLOAT) * (in_circle as FLOAT) / ((nthreads * samples) as FLOAT);
    println!("{}", pi);
}

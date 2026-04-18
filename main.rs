mod payload;

use chrono::Local;
use core::arch::x86_64::_rdtsc;
use payload::*;
use std::{fs::File, hint::black_box, io::Write};

/// Ramp up the compute path to remove over time optimizations. (e.g. cache)
const WARM_UP_ITERATIONS: usize = 2_000_000;
const ITERATIONS: usize = 20_000_000;

fn main() {
    let mut latencies: Vec<u64> = Vec::new();
    let payload_one = One;
    let payload_thirty_two = ThirtyTwo;

    let total_iterations = WARM_UP_ITERATIONS + ITERATIONS;

    for i in 0..total_iterations {
        let latency = unsafe {
            let start = _rdtsc();

            static_dispatch(black_box(&payload_thirty_two));

            let end = _rdtsc();

            end - start
        };

        if i >= WARM_UP_ITERATIONS {
            latencies.push(latency);
        }
    }

    let timestamp = Local::now().format("%s");
    let filename = format!("data-{timestamp}.csv");
    let mut file = File::create(filename).unwrap();
    writeln!(file, "cycles").unwrap();

    // Sort in rust so it's speeeeeeeed.
    latencies.sort();

    for cycles in latencies {
        writeln!(file, "{cycles}").unwrap();
    }
}

fn static_dispatch(payload: &impl Payload) {
    payload.run();
}

fn dynamic_dispatch(payload: &dyn Payload) {
    payload.run();
}

mod payload;

use chrono::Local;
use core::arch::x86_64::_rdtsc;
use payload::*;
use std::{fs::File, io::Write};

const ITERATIONS: usize = 20_000_000;
// Approximate based on DDR5.
const CLOCK_CYCLE_NS: f64 = 0.357;

fn main() {
    let mut latencies: Vec<u64> = Vec::new();
    let payload_one = One;
    let payload_sixteen = Sixteen;

    for _ in 0..ITERATIONS {
        let latency = unsafe {
            let start = _rdtsc();

            static_dispatch(&payload_sixteen);

            let end = _rdtsc();

            end - start
        };

        latencies.push(latency);
    }

    let timestamp = Local::now().format("%s");
    let filename = format!("data-{timestamp}.csv");
    let mut file = File::create(filename).unwrap();
    writeln!(file, "latency_ns").unwrap();

    // Sort in rust so it's speeeeeeeed.
    latencies.sort();

    for latency in latencies {
        let latency_ns = (latency as f64) * CLOCK_CYCLE_NS;
        writeln!(file, "{latency_ns}").unwrap();
    }
}

#[allow(unused)]
fn static_dispatch(payload: &impl Payload) {
    payload.run()
}

#[allow(unused)]
fn dynamic_dispatch(payload: &dyn Payload) {
    payload.run()
}

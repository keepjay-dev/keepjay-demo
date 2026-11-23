//! An integration test with real waiting: a client retries against a server that fails
//! its first six requests, sleeping for each backoff delay in between. Like tests against
//! real services, it's slow (about 13 seconds), which is when `kj while -- cargo test` helps.

use std::cell::Cell;
use std::time::{Duration, Instant};

use keepjay_demo::backoff::Backoff;
use rand::SeedableRng;
use rand::rngs::StdRng;

/// Fails the first `failures` calls, then succeeds.
struct FlakyServer {
    failures: u32,
    calls: Cell<u32>,
}

impl FlakyServer {
    fn request(&self) -> Result<&'static str, &'static str> {
        self.calls.set(self.calls.get() + 1);
        if self.calls.get() <= self.failures {
            Err("503 Service Unavailable")
        } else {
            Ok("200 OK")
        }
    }
}

#[test]
fn retries_until_the_server_recovers() {
    let server = FlakyServer {
        failures: 6,
        calls: Cell::new(0),
    };
    let backoff = Backoff::from_json(r#"{"base_ms": 400, "factor": 2, "max_ms": 6000}"#).unwrap();
    let mut rng = StdRng::seed_from_u64(4); // a fixed seed, so the test always takes as long
    let started = Instant::now();

    let mut attempt = 0;
    let response = loop {
        match server.request() {
            Ok(response) => break response,
            Err(_) => {
                std::thread::sleep(backoff.delay(attempt, &mut rng));
                attempt += 1;
            }
        }
    };

    assert_eq!(response, "200 OK");
    assert_eq!(server.calls.get(), 7);
    assert!(started.elapsed() < Duration::from_secs(6 * 6), "every wait is capped");
}

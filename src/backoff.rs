//! Retry delays: exponential backoff with full jitter.

use std::time::Duration;

use rand::Rng;
use serde::Deserialize;

/// How long to wait before each retry.
pub struct Backoff {
    pub base: Duration,
    pub factor: u32,
    pub max: Duration,
}

/// Backoff settings as written in a config file: `{"base_ms": 100, "factor": 2, "max_ms": 5000}`.
#[derive(Deserialize)]
struct Config {
    base_ms: u64,
    factor: u32,
    max_ms: u64,
}

impl Backoff {
    pub fn from_json(text: &str) -> serde_json::Result<Self> {
        let c: Config = serde_json::from_str(text)?;
        Ok(Backoff {
            base: Duration::from_millis(c.base_ms),
            factor: c.factor,
            max: Duration::from_millis(c.max_ms),
        })
    }

    // @card Why does `delay` wait a random time up to the cap, not the cap itself?
    // So clients that failed together don't retry together: "full jitter" spreads
    // the retries out instead of sending a thundering herd at a recovering server.
    pub fn delay(&self, attempt: u32, rng: &mut impl Rng) -> Duration {
        let cap = self.base.saturating_mul(self.factor.saturating_pow(attempt));
        let cap = cap.min(self.max).as_millis() as u64;
        Duration::from_millis(rng.random_range(0..=cap))
    }
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    use super::*;

    const B: Backoff = Backoff {
        base: Duration::from_millis(100),
        factor: 2,
        max: Duration::from_secs(5),
    };

    #[test]
    fn reads_its_settings_from_json() {
        let b = Backoff::from_json(r#"{"base_ms": 100, "factor": 2, "max_ms": 5000}"#).unwrap();
        assert_eq!((b.base, b.factor, b.max), (B.base, B.factor, B.max));
    }

    #[test]
    fn delays_stay_under_the_cap() {
        let mut rng = StdRng::seed_from_u64(7);
        for attempt in 0..20 {
            let cap = (100 * 2u64.saturating_pow(attempt)).min(5_000);
            assert!(B.delay(attempt, &mut rng).as_millis() as u64 <= cap);
        }
    }

    #[test]
    fn delays_are_spread_out() {
        let mut rng = StdRng::seed_from_u64(7);
        let delays: Vec<_> = (0..50).map(|_| B.delay(6, &mut rng)).collect();
        assert!(delays.iter().any(|d| *d < Duration::from_secs(2)));
        assert!(delays.iter().any(|d| *d > Duration::from_secs(3)));
    }
}

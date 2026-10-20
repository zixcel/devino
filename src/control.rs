use crate::Error;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::time::{Duration, Instant};
/// Cooperative native cancellation. Process termination belongs to the caller.
#[derive(Clone)]
pub struct GenerationControl {
    cancelled: Arc<AtomicBool>,
    deadline: Instant,
}
impl GenerationControl {
    /// Create one finite budget; observations never renew it.
    /// # Errors
    /// Rejects zero and budgets above thirty seconds.
    pub fn new(budget_ms: u32) -> Result<Self, Error> {
        Self::at(budget_ms, Instant::now())
    }
    fn at(budget_ms: u32, started: Instant) -> Result<Self, Error> {
        if !(1..=30000).contains(&budget_ms) {
            return Err(Error::InvalidConfiguration);
        }
        Ok(Self {
            cancelled: Arc::new(AtomicBool::new(false)),
            deadline: started + Duration::from_millis(u64::from(budget_ms)),
        })
    }
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }
    /// # Errors
    /// Cancellation and the original monotonic deadline remain terminal.
    pub fn check(&self) -> Result<(), Error> {
        self.check_at(Instant::now())
    }
    fn check_at(&self, observed: Instant) -> Result<(), Error> {
        if self.cancelled.load(Ordering::Acquire) {
            return Err(Error::Cancelled);
        }
        if observed >= self.deadline {
            return Err(Error::DeadlineExceeded);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deadline_is_exact_and_observation_cannot_renew_it_without_sleeping() {
        let start = Instant::now();
        let control = GenerationControl::at(10, start).expect("finite budget");
        assert_eq!(control.check_at(start + Duration::from_millis(9)), Ok(()));
        assert_eq!(
            control.check_at(start + Duration::from_millis(10)),
            Err(Error::DeadlineExceeded)
        );
        assert_eq!(
            control.clone().check_at(start + Duration::from_millis(11)),
            Err(Error::DeadlineExceeded)
        );
    }
    #[test]
    fn cancellation_wins_without_signals_or_native_work() {
        let start = Instant::now();
        let control = GenerationControl::at(10, start).expect("budget");
        control.cancel();
        assert_eq!(control.check_at(start), Err(Error::Cancelled));
        assert_eq!(
            control.check_at(start + Duration::from_secs(1)),
            Err(Error::Cancelled)
        );
    }
}

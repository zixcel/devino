use crate::{Error, GenerationControl, GenerationOptions};
use openvino_genai::{Streamer, StreamingStatus};
use std::sync::{Arc, LockResult, Mutex, MutexGuard};
struct Budget {
    caller: GenerationControl,
    local: GenerationControl,
    bytes: usize,
    maximum: usize,
    failed: Option<Error>,
}
impl Budget {
    fn observe(&mut self, chunk: &str) -> Result<(), Error> {
        if let Some(error) = self.failed {
            return Err(error);
        }
        let result = self
            .caller
            .check()
            .and_then(|()| self.local.check())
            .and_then(|()| {
                self.bytes = self.bytes.saturating_add(chunk.len());
                if self.bytes > self.maximum {
                    Err(Error::OutputLimitExceeded)
                } else {
                    Ok(())
                }
            });
        if let Err(error) = result {
            self.failed = Some(error);
        }
        result
    }
}
pub(crate) struct OutputMonitor(Arc<Mutex<Budget>>);
impl OutputMonitor {
    pub(crate) fn new(
        options: &GenerationOptions,
        caller: GenerationControl,
    ) -> Result<Self, Error> {
        Ok(Self(Arc::new(Mutex::new(Budget {
            caller,
            local: GenerationControl::new(options.budget_ms)?,
            bytes: 0,
            maximum: options.max_output_bytes as usize,
            failed: None,
        }))))
    }
    pub(crate) fn streamer(&self) -> Streamer {
        let state = Arc::clone(&self.0);
        Streamer::new(move |chunk| match access(state.lock(), chunk) {
            Ok(()) => StreamingStatus::Running,
            Err(_) => StreamingStatus::Cancel,
        })
    }
    pub(crate) fn finish(&self) -> Result<(), Error> {
        access(self.0.lock(), "")
    }
}
fn access(state: LockResult<MutexGuard<'_, Budget>>, chunk: &str) -> Result<(), Error> {
    state.map_err(|_| Error::BackendFailed)?.observe(chunk)
}
#[cfg(test)]
mod tests {
    use super::*;
    fn monitor() -> OutputMonitor {
        OutputMonitor::new(
            &GenerationOptions {
                max_new_tokens: 16,
                max_total_tokens: 1024,
                max_output_bytes: 4,
                budget_ms: 1000,
            },
            GenerationControl::new(1000).expect("budget"),
        )
        .expect("monitor")
    }
    #[test]
    fn byte_limit_is_terminal_and_counts_utf8_not_callback_or_token_counts() {
        let m = monitor();
        let mut s = m.0.lock().expect("state");
        assert_eq!(s.observe("あ"), Ok(()));
        assert_eq!(s.observe("ab"), Err(Error::OutputLimitExceeded));
        assert_eq!(s.observe(""), Err(Error::OutputLimitExceeded));
    }
    #[test]
    fn unavailable_callback_state_cancels_without_panicking_or_spawning_threads() {
        let m = monitor();
        let guard = m.0.lock().expect("state");
        let unavailable = Err(std::sync::PoisonError::new(guard));
        assert_eq!(access(unavailable, ""), Err(Error::BackendFailed));
    }
}

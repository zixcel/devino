use std::fmt;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    ProcessIsolationRequired,
    InvalidConfiguration,
    InvalidModel,
    IntegrityFailed,
    RuntimeUnavailable,
    RuntimeAlreadySelected,
    BackendFailed,
    Cancelled,
    DeadlineExceeded,
    OutputLimitExceeded,
    UsageInvalid,
}
impl Error {
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::ProcessIsolationRequired => "backend-process-isolation-required",
            Self::InvalidConfiguration => "invalid-configuration",
            Self::InvalidModel => "invalid-model-bundle",
            Self::IntegrityFailed => "integrity-failed",
            Self::RuntimeUnavailable => "runtime-unavailable",
            Self::RuntimeAlreadySelected => "runtime-already-selected",
            Self::BackendFailed => "backend-failed",
            Self::Cancelled => "cancelled",
            Self::DeadlineExceeded => "deadline-exceeded",
            Self::OutputLimitExceeded => "output-limit-exceeded",
            Self::UsageInvalid => "usage-invalid",
        }
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}
impl std::error::Error for Error {}

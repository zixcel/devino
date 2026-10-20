//! OpenVINO-specific execution, without a model store, downloader or supervisor.
#![forbid(unsafe_code)]
mod configuration;
mod control;
mod error;
mod generation;
mod monitor;
mod native_libraries;
mod observation;
mod runtime;
pub use configuration::GenerationOptions;
pub use control::GenerationControl;
pub use error::Error;
pub use generation::OpenVinoCpu;
pub use native_libraries::NativeLibraries;
pub use observation::GenerationObservation;
pub use runtime::OpenVinoRuntime;

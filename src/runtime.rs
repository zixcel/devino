use crate::{Error, NativeLibraries};
use std::sync::Mutex;
struct Selected {
    libraries: NativeLibraries,
    ready: bool,
}
static SELECTED: Mutex<Option<Selected>> = Mutex::new(None);
pub struct OpenVinoRuntime {
    pub(crate) context: zixcel_local_inference::backend_process::BackendProcessContext,
}
impl OpenVinoRuntime {
    /// Load explicit native entry libraries once in the caller-owned process.
    /// The caller must isolate and admit the full native distribution first.
    /// # Errors
    /// Rejects integrity errors and any second native-library selection.
    pub fn load(
        context: &zixcel_local_inference::backend_process::BackendProcessContext,
        libraries: &NativeLibraries,
    ) -> Result<Self, Error> {
        context
            .check()
            .map_err(|_| Error::ProcessIsolationRequired)?;
        libraries.verify()?;
        let mut selected = SELECTED.lock().map_err(|_| Error::RuntimeUnavailable)?;
        if let Some(previous) = selected.as_ref() {
            return if previous.libraries == *libraries && previous.ready {
                Ok(Self {
                    context: context.clone(),
                })
            } else {
                Err(Error::RuntimeAlreadySelected)
            };
        }
        // Reserve even after partial loading: a process must never mix distributions.
        *selected = Some(Selected {
            libraries: libraries.clone(),
            ready: false,
        });
        openvino_sys::library::load_from(&libraries.openvino_c)
            .map_err(|_| Error::RuntimeUnavailable)?;
        openvino_genai::load_from(&libraries.genai_c).map_err(|_| Error::RuntimeUnavailable)?;
        if let Some(selected) = selected.as_mut() {
            selected.ready = true;
        }
        Ok(Self {
            context: context.clone(),
        })
    }
    /// Observe native device names, without automatic device fallback.
    /// # Errors
    /// Returns a stable backend error, not raw native diagnostics.
    pub fn devices(&self) -> Result<Vec<String>, Error> {
        self.context
            .check()
            .map_err(|_| Error::ProcessIsolationRequired)?;
        openvino::Core::new()
            .map_err(|_| Error::BackendFailed)?
            .available_devices()
            .map_err(|_| Error::BackendFailed)
            .map(|devices| {
                devices
                    .into_iter()
                    .map(|device| device.to_string())
                    .collect()
            })
    }
}

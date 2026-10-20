use crate::Error;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GenerationOptions {
    pub max_new_tokens: u16,
    pub max_total_tokens: u32,
    pub max_output_bytes: u32,
    pub budget_ms: u32,
}
impl GenerationOptions {
    /// Validate a bounded prompt and deterministic generation configuration.
    /// # Errors
    /// Rejects NUL, oversized input and unsupported resource limits.
    pub fn validate(&self, prompt: &str) -> Result<(), Error> {
        if prompt.is_empty()
            || prompt.len() > 4096
            || prompt.contains('\0')
            || !(1..=256).contains(&self.max_new_tokens)
            || self.max_total_tokens < u32::from(self.max_new_tokens)
            || self.max_total_tokens > 8192
            || !(1..=65536).contains(&self.max_output_bytes)
            || !(1..=30000).contains(&self.budget_ms)
        {
            return Err(Error::InvalidConfiguration);
        }
        Ok(())
    }
}

impl GenerationOptions {
    pub(crate) fn native_config(&self) -> Result<openvino_genai::GenerationConfig, Error> {
        let mut config =
            openvino_genai::GenerationConfig::new().map_err(|_| Error::BackendFailed)?;
        config
            .set_do_sample(false)
            .map_err(|_| Error::BackendFailed)?;
        config.set_num_beams(1).map_err(|_| Error::BackendFailed)?;
        config
            .set_max_new_tokens(usize::from(self.max_new_tokens))
            .map_err(|_| Error::BackendFailed)?;
        config
            .set_max_length(self.max_total_tokens as usize)
            .map_err(|_| Error::BackendFailed)?;
        config.validate().map_err(|_| Error::BackendFailed)?;
        Ok(config)
    }
}

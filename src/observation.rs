use serde::Serialize;
/// Backend-local observation; never a process/adoption receipt.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerationObservation {
    pub output: String,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub elapsed_ms: u64,
    pub model_manifest_sha256: String,
}

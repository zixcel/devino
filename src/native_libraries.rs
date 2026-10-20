use crate::Error;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fs::OpenOptions,
    io::Read,
    path::{Path, PathBuf},
};
/// Explicit native library identities. The external owner admits dependencies/plugins.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NativeLibraries {
    pub openvino_c: PathBuf,
    pub openvino_sha256: String,
    pub genai_c: PathBuf,
    pub genai_sha256: String,
}
fn verify(path: &Path, expected: &str) -> Result<(), Error> {
    if !path.is_absolute()
        || expected.len() != 64
        || !expected
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err(Error::InvalidConfiguration);
    }
    for part in path.ancestors() {
        if std::fs::symlink_metadata(part)
            .map_err(|_| Error::IntegrityFailed)?
            .file_type()
            .is_symlink()
        {
            return Err(Error::IntegrityFailed);
        }
    }
    let mut options = OpenOptions::new();
    options.read(true);
    #[cfg(target_os = "linux")]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(0x800 | 0x20000);
    }
    let mut file = options.open(path).map_err(|_| Error::IntegrityFailed)?;
    let metadata = file.metadata().map_err(|_| Error::IntegrityFailed)?;
    if !metadata.is_file() || metadata.len() == 0 || metadata.len() > 1024 * 1024 * 1024 {
        return Err(Error::IntegrityFailed);
    }
    let mut hash = Sha256::new();
    let mut buffer = [0_u8; 8192];
    let mut total = 0_u64;
    loop {
        let n = file.read(&mut buffer).map_err(|_| Error::IntegrityFailed)?;
        if n == 0 {
            break;
        }
        total += n as u64;
        if total > metadata.len() {
            return Err(Error::IntegrityFailed);
        }
        hash.update(&buffer[..n]);
    }
    if total != metadata.len() || hex(&hash.finalize()) != expected {
        return Err(Error::IntegrityFailed);
    }
    Ok(())
}
impl NativeLibraries {
    /// Hash only; does not load code or attest native dependency closure.
    /// # Errors
    /// Rejects paths, links, nonregular files and digest mismatches.
    pub fn verify(&self) -> Result<(), Error> {
        verify(&self.openvino_c, &self.openvino_sha256)?;
        verify(&self.genai_c, &self.genai_sha256)
    }
}

fn hex(bytes: &[u8]) -> String {
    use std::fmt::Write;
    let mut text = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        let _ = write!(text, "{byte:02x}");
    }
    text
}

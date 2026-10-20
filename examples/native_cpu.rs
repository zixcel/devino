//! Explicit opt-in native acceptance, never a skipped default test.
#[path = "native_cpu/worker.rs"]
mod worker;
use std::{
    error::Error,
    os::unix::process::CommandExt,
    process::Command,
    time::{Duration, Instant},
};
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().is_some_and(|arg| arg == "--worker") {
        return worker::run(&args[1..]);
    }
    if args.len() != 5 {
        return Err("expected: bundle-root manifest-file manifest-sha256 libraries-file native-library-path".into());
    }
    // The explicit distribution path is selected by the operator, not discovered.
    for path in args[4].split(':') {
        if !std::path::Path::new(path).is_absolute() || !std::path::Path::new(path).is_dir() {
            return Err("native-library-path must contain existing absolute directories".into());
        }
    }
    let mut child = Command::new(std::env::current_exe()?)
        .arg("--worker")
        .arg(std::process::id().to_string())
        .args(&args[..4])
        .env_clear()
        .env("LD_LIBRARY_PATH", &args[4])
        .env("OMP_NUM_THREADS", "1")
        .env("OPENBLAS_NUM_THREADS", "1")
        .stdin(std::process::Stdio::null())
        .process_group(0)
        .spawn()?;
    let end = Instant::now() + Duration::from_secs(45);
    loop {
        if let Some(status) = child.try_wait()? {
            if !status.success() {
                return Err(format!("native worker rejected: {status}").into());
            }
            return Ok(());
        }
        if Instant::now() >= end {
            // Exact, unreaped Child ownership; never signal the parent or an ambient PID.
            let _ = child.kill();
            child.wait()?;
            return Err("native acceptance deadline exceeded".into());
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

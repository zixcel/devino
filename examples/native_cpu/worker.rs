use devino::{GenerationControl, GenerationOptions, NativeLibraries, OpenVinoCpu, OpenVinoRuntime};
use std::{error::Error, path::Path, time::Duration};
use zixcel_local_inference::{
    VerificationControl,
    backend_process::{BackendProcessContext, WorkerLimits},
    model_bundle::VerifiedModelBundle,
};
pub fn run(args: &[String]) -> Result<(), Box<dyn Error>> {
    if args.len() != 5 {
        return Err("invalid native worker arguments".into());
    }
    let context = BackendProcessContext::attach(
        args[0].parse()?,
        WorkerLimits {
            address_space_bytes: 2 * 1024 * 1024 * 1024,
            cpu_seconds: 20,
        },
    )?;
    let libraries: NativeLibraries = serde_json::from_slice(&std::fs::read(&args[4])?)?;
    let runtime = OpenVinoRuntime::load(&context, &libraries)?;
    if !runtime.devices()?.iter().any(|device| device == "CPU") {
        return Err("CPU device unavailable".into());
    }
    let document = std::fs::read(&args[2])?;
    let bundle = VerifiedModelBundle::verify(
        Path::new(&args[1]),
        &document,
        &args[3],
        &VerificationControl::new(Duration::from_secs(5)),
    )?;
    let mut pipeline = OpenVinoCpu::load(&runtime, bundle)?;
    let options = GenerationOptions {
        max_new_tokens: 8,
        max_total_tokens: 64,
        max_output_bytes: 1024,
        budget_ms: 5000,
    };
    let first = pipeline.generate("Acceptance test", &options, &GenerationControl::new(5000)?)?;
    let second = pipeline.generate("Acceptance test", &options, &GenerationControl::new(5000)?)?;
    if first.output != second.output || first.input_tokens == 0 || first.output_tokens != 8 {
        return Err("deterministic generation/usage mismatch".into());
    }
    let cancelled = GenerationControl::new(5000)?;
    cancelled.cancel();
    if pipeline
        .generate("Acceptance test", &options, &cancelled)
        .err()
        != Some(devino::Error::Cancelled)
    {
        return Err("pre-cancelled native request was not rejected".into());
    }
    let limited = GenerationOptions {
        max_output_bytes: 1,
        ..options.clone()
    };
    if pipeline
        .generate("Acceptance test", &limited, &GenerationControl::new(5000)?)
        .err()
        != Some(devino::Error::OutputLimitExceeded)
    {
        return Err("native output budget was not enforced".into());
    }
    let recovered =
        pipeline.generate("Acceptance test", &options, &GenerationControl::new(5000)?)?;
    if recovered.output != first.output {
        return Err("native pipeline did not recover after output cancellation".into());
    }
    println!(
        "{}",
        serde_json::json!({"state":"complete", "device":"CPU",
        "inputTokens":first.input_tokens,"outputTokens":first.output_tokens,
        "output":first.output,"repeatMatched":true,"preCancelledRejected":true,"outputLimitRejected":true,"postCancelRecovery":true})
    );
    Ok(())
}

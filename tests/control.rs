use devino::{Error, GenerationControl, GenerationOptions, NativeLibraries};
fn options() -> GenerationOptions {
    GenerationOptions {
        max_new_tokens: 16,
        max_total_tokens: 1024,
        max_output_bytes: 4096,
        budget_ms: 1000,
    }
}
#[test]
fn bounded_options_accept_plain_input_and_reject_nul_and_resource_overruns() {
    assert!(options().validate("hello").is_ok());
    for prompt in ["", "text\0hidden", &"a".repeat(4097)] {
        assert_eq!(options().validate(prompt), Err(Error::InvalidConfiguration));
    }
    for (tokens, total, bytes, budget) in [
        (0, 1024, 4096, 1000),
        (257, 1024, 4096, 1000),
        (16, 2, 4096, 1000),
        (16, 8193, 4096, 1000),
        (16, 1024, 65537, 1000),
        (16, 1024, 4096, 30001),
    ] {
        let selected = GenerationOptions {
            max_new_tokens: tokens,
            max_total_tokens: total,
            max_output_bytes: bytes,
            budget_ms: budget,
        };
        assert_eq!(selected.validate("hello"), Err(Error::InvalidConfiguration));
    }
}
#[test]
fn cancellation_is_shared_terminal_and_does_not_change_process_or_start_runtime() {
    let pid = std::process::id();
    let control = GenerationControl::new(1000).expect("budget");
    let observer = control.clone();
    assert_eq!(observer.check(), Ok(()));
    control.cancel();
    assert_eq!(observer.check(), Err(Error::Cancelled));
    assert_eq!(control.check(), Err(Error::Cancelled));
    assert_eq!(std::process::id(), pid);
}
#[test]
fn zero_and_unbounded_deadlines_reject_without_starting_a_timer() {
    assert!(GenerationControl::new(0).is_err());
    assert!(GenerationControl::new(30001).is_err());
}

#[test]
fn native_verification_rejects_unpinned_or_missing_library_without_loading_it() {
    let libraries = NativeLibraries {
        openvino_c: "/not-installed/libopenvino_c.so".into(),
        genai_c: "/not-installed/libgenai.so".into(),
        openvino_sha256: "0".repeat(64),
        genai_sha256: "0".repeat(64),
    };
    assert_eq!(libraries.verify(), Err(Error::IntegrityFailed));
    let invalid = NativeLibraries {
        openvino_sha256: "unverified".into(),
        ..libraries
    };
    assert_eq!(invalid.verify(), Err(Error::InvalidConfiguration));
}

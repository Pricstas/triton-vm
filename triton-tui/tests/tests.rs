use std::path::PathBuf;

use assert2::let_assert;
use rexpect::error::Error;
use rexpect::session::PtySession;
use rexpect::spawn;

#[test]
#[ignore = "breaks code-coverage tool `cargo-tarpaulin`"]
fn setup_and_shutdown_triton_tui_with_trivial_program() {
    let timeout = Some(180_000);
    let mut child = setup_and_start_triton_tui_with_trivial_program(timeout);
    let_assert!(Ok(_) = child.send_line("q"));
    let_assert!(Ok(_) = child.exp_eof());
}

#[test]
#[ignore = "breaks code-coverage tool `cargo-tarpaulin`"]
fn setup_without_shutdown_of_triton_tui_with_trivial_program_leaves_tui_open() {
    let timeout = Some(10_000);
    let mut child = setup_and_start_triton_tui_with_trivial_program(timeout);
    let_assert!(Err(Error::Timeout { .. }) = child.exp_eof());
}

fn setup_and_start_triton_tui_with_trivial_program(timeout: Option<u64>) -> PtySession {
    let path_to_trivial_program = manifest_dir().join("tests/trivial_program.tasm");
    assert!(path_to_trivial_program.exists());
    let_assert!(Some(path_to_trivial_program) = path_to_trivial_program.to_str());

    let command = format!("cargo run --offline --bin triton-tui -- {path_to_trivial_program}");
    let_assert!(Ok(child) = spawn(&command, timeout));
    child
}

#[test]
#[ignore = "breaks code-coverage tool `cargo-tarpaulin`"]
fn setup_and_shutdown_triton_tui_with_example_initial_state() {
    let timeout = Some(180_000);
    let mut child = setup_and_start_triton_tui_with_example_initial_state(timeout);
    let_assert!(Ok(_) = child.send_line("q"));
    let_assert!(Ok(_) = child.exp_eof());
}

fn setup_and_start_triton_tui_with_example_initial_state(timeout: Option<u64>) -> PtySession {
    let path_to_trivial_program = manifest_dir().join("examples/program.tasm");
    assert!(path_to_trivial_program.exists());
    let_assert!(Some(path_to_trivial_program) = path_to_trivial_program.to_str());

    let path_to_initial_state = manifest_dir().join("examples/initial_state.json");
    assert!(path_to_initial_state.exists());
    let_assert!(Some(path_to_initial_state) = path_to_initial_state.to_str());

    let command = format!(
        "cargo run --offline --bin triton-tui -- \
        {path_to_trivial_program} --initial_state {path_to_initial_state}"
    );
    let_assert!(Ok(child) = spawn(&command, timeout));
    child
}

/// The directory containing the Cargo.toml file.
fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

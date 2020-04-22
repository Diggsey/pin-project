#![warn(unsafe_code)]
#![warn(rust_2018_idioms, single_use_lifetimes)]

// Based on https://github.com/serde-rs/serde/blob/v1.0.106/test_suite/build.rs

use std::process::{Command, ExitStatus, Stdio};

#[cfg(not(windows))]
const CARGO_EXPAND: &str = "cargo-expand";

#[cfg(windows)]
const CARGO_EXPAND: &str = "cargo-expand.exe";

fn main() {
    if Command::new(CARGO_EXPAND)
        .arg("--version")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .as_ref()
        .map(ExitStatus::success)
        .unwrap_or(false)
    {
        println!("cargo:rustc-cfg=cargo_expand");
    }
}

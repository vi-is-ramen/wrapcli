//! Example: `cargo run --example basic -- -V`
//!
//! Runs `rustc -V` through wrapcli and shows how changes
//! first output string.

use wrapcli::{run_streaming, WrapConfig};

fn main() -> std::io::Result<()> {
    let cfg = WrapConfig {
        orig_name: "rustc".into(),
        fake_name: "dustc".into(),
        fake_ver: "2.0.0".into(),
        save_orig: true,
    };

    let args: Vec<String> = std::env::args().skip(1).collect();
    let status = run_streaming(&cfg, args)?;
    std::process::exit(status.code().unwrap_or(1));
}

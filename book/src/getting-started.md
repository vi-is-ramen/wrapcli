# Getting Started

## Installation

Add `wrapcli` to your `Cargo.toml`:

```shell
cargo add wrapcli
```

Or add it manually:

```toml
[dependencies]
wrapcli = "0.1"
```

## Basic Usage

```rust
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
```

## How It Works

When the wrapped tool outputs a line containing the original name, `wrapcli` intercepts it and applies rewriting rules:

1. The **first occurrence** of the original tool's name and version is replaced with the fake name and fake version. If `save_orig` is `true`, the original version is appended in parentheses.
2. Any **subsequent occurrences** of the original name in usage lines are replaced with the fake name.

This ensures consistent output masking without breaking the tool's functionality.

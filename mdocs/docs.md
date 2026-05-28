# wrapcli

**CLI identity faking utility.**

[![CI](https://github.com/vi-is-ramen/wrapcli/workflows/CI/badge.svg)](https://github.com/vi-is-ramen/wrapcli/actions)
[![Crates.io](https://img.shields.io/crates/v/wrapcli.svg)](https://crates.io/crates/wrapcli)
[![Docs.rs](https://docs.rs/wrapcli/badge.svg)](https://docs.rs/wrapcli)

[Docs](../index/index.html)

## Usage

Add wrapcli to your project's dependencies:

```toml
[dependencies]
wrapcli = "0.1.0"
```

Then:

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

More examples can be found in the [examples](./examples) directory.

## License

Licensed under MIT OR Apache-2.0

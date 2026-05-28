# wrapcli

[![Crates.io](https://img.shields.io/crates/v/wrapcli.svg)](https://crates.io/crates/wrapcli)
[![Docs.rs](https://docs.rs/wrapcli/badge.svg)](https://docs.rs/wrapcli)
[![CI](https://github.com/@author@/wrapcli/workflows/CI/badge.svg)](https://github.com/@author@/wrapcli/actions)

CLI identity faking utility.

## Usage

Add this to your project's dependencies:

```shell
cargo add wrapcli
```

Then use it in your code:

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

## Documentation

Full documentation is available on [docs.rs](https://docs.rs/wrapcli).

## License

Licensed under MIT.

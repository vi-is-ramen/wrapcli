# wrapcli

Wrap a CLI tool and rewrite its output to masquerade as another tool.

[![Crates.io](https://img.shields.io/crates/v/wrapcli.svg)](https://crates.io/crates/wrapcli)
[![Docs.rs](https://docs.rs/wrapcli/badge.svg)](https://docs.rs/wrapcli)
[![CI](https://github.com/vi-is-ramen/wrapcli/actions/workflows/ci.yml/badge.svg)](https://github.com/vi-is-ramen/wrapcli/actions/workflows/ci.yml)

## Quick start

Add to your project:

```shell
cargo add wrapcli
```

Then use it:

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

Full documentation is available in the [WrapCLI Book](https://vi-is-ramen.github.io/wrapcli).

## License

MIT OR Apache-2.0

---

*Made with ❤️ for the Rust community*

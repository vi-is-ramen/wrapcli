# Examples

## Streaming Output

```rust
use wrapcli::{run_streaming, WrapConfig};

fn main() -> std::io::Result<()> {
    let cfg = WrapConfig {
        orig_name: "cargo".into(),
        fake_name: "pargo".into(),
        fake_ver: "2.0.0".into(),
        save_orig: true,
    };

    let args = vec!["--version".to_string()];
    run_streaming(&cfg, args)?;
    Ok(())
}
```

## Capturing Output

```rust
use wrapcli::{run_capture, WrapConfig};

fn main() -> std::io::Result<()> {
    let cfg = WrapConfig {
        orig_name: "rustc".into(),
        fake_name: "dustc".into(),
        fake_ver: "2.0.0".into(),
        save_orig: false,
    };

    let args = vec!["--version".to_string()];
    let result = run_capture(&cfg, args)?;
    
    println!("Captured stdout: {}", String::from_utf8_lossy(&result.stdout));
    println!("Captured stderr: {}", String::from_utf8_lossy(&result.stderr));
    Ok(())
}
```

## Real-World Example: Wrapping Git

```rust
use wrapcli::{run_streaming, WrapConfig};
use std::env;

fn main() -> std::io::Result<()> {
    let cfg = WrapConfig {
        orig_name: "git".into(),
        fake_name: "gitter".into(),
        fake_ver: "3.0.0".into(),
        save_orig: true,
    };

    let args: Vec<String> = env::args().skip(1).collect();
    run_streaming(&cfg, args)
}
```

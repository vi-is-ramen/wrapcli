# @project@

**@description@**

[![CI](@repo@/workflows/CI/badge.svg)](@repo@/actions)
[![Crates.io](https://img.shields.io/crates/v/@project@.svg)](https://crates.io/crates/@project@)
[![Docs.rs](https://docs.rs/@project@/badge.svg)](https://docs.rs/@project@)

## Usage

Add @project@ to your project's dependencies:

```toml
[dependencies]
@project@ = "0.1.0"
```

Then:

```rust
use @project@::greet;

fn main() {
    println!("{}", greet("world"));
}
```

More examples can be found in the [examples](./examples) directory.

## License

Licensed under @license@

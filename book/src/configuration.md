# Configuration

The `WrapConfig` struct controls the rewriting behavior.

## Fields

| Field | Type | Description |
|-------|------|-------------|
| `orig_name` | `String` | The original tool's name (e.g., `"rustc"`) |
| `fake_name` | `String` | The name to display instead (e.g., `"dustc"`) |
| `fake_ver` | `String` | The version string to display (e.g., `"2.0.0"`) |
| `save_orig` | `bool` | Whether to append the original version in parentheses |

## Example

```rust
use wrapcli::WrapConfig;

let cfg = WrapConfig {
    orig_name: "git".into(),
    fake_name: "gitter".into(),
    fake_ver: "3.0.0".into(),
    save_orig: false,
};
```

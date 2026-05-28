# wrapcli

Runs extern command and changes its output:
- command name (`orig_name` -> `fake_name`);
- version (`<any>` -> `fake_ver`);

Replacement rules:
- First line: if starts with `<orig_name> <version>`,
  both values'll be replaced. If contains `Usage: <orig_name>`,
  only nema'll be replaced.
- Rest lines: changed `Usage: <orig_name>` only (if present).

Rest `orig_name` (in diagnostics, error reports and so on) still same.

How to use:
1. Create [`crate::WrapConfig`] object;
2. Create [`Vec<String>`] object containing command arguments;
3. Call [`crate::run_streaming`] function with your config and arguments;
4. PROFIT!

# Introduction

**wrapcli** is a Rust library that wraps an existing CLI tool and rewrites its output on the fly, making it appear as if a different tool produced the output.

## Features

- Stream real-time output rewriting (line by line)
- Capture full output for post-processing
- Customizable rewriting rules
- Preserve original version information (optional)

## Use Cases

- Create branded variants of CLI tools
- Mask internal tool names in user-facing output
- Add version information without changing the underlying tool
- Integrate legacy tools into modern workflows

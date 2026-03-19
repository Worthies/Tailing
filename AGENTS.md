# Agentic Coding Guidelines

## Project Overview
- **Project Name**: tail
- **Type**: CLI tool (Rust)
- **Core Functionality**: A drop-in replacement for system `tail` command that also logs output to systemd journal

## Development Workflow

### Building
```bash
cargo build
```

### Testing
```bash
cargo test
```

### Running
```bash
# Basic usage
echo "test" | tail

# With tail arguments
echo "test" | tail -n 10
```

### Checking Journal Logs
```bash
journalctl -f -t tail-<PID>
```

## Code Style
- Follow Rust standard formatting (`cargo fmt`)
- Use `cargo clippy` for linting
- Keep dependencies minimal

## Architecture
- Main entry point in `src/main.rs`
- Uses `std::process::Command` to invoke `/usr/bin/logger` and `/usr/bin/tail`
- Streams stdin line-by-line for real-time logging

## Key Implementation Details
1. Get parent PID to create unique tag `tail-<PPID>`
2. Spawn journal logger process
3. Spawn tail process with passed arguments
4. Stream input to both processes in real-time
5. Print tag to stderr for identification

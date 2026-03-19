# Rust tail Implementation Plan

## Project Overview
- **Project Name**: tail
- **Type**: CLI tool (Rust)
- **Install name**: `tail` (via `cargo install --github github.com/org/tailing`)
- **Core Functionality**: Read stdin/input, log to systemd journal with tag 'tail-<ppid>', print with tag prefix
- **Target Users**: System administrators, developers who need to pipe output to system logger

## Functionality Specification

### Core Features
1. **Read Input**: Read all input from stdin (supports piping from other commands)
2. **Generate Tag**: Create tag in format `tail-<ppid>` where ppid is the parent's process ID
3. **Log to systemd journal**: Use `systemd` crate to log content directly
4. **Print with Tag**: Output each line prefixed with the tag

### User Interactions
- Input: Read from stdin (piped content)
- Output: Print each line with `<tag>` prefix
- Logger: Send to systemd journal with SYSLOG_IDENTIFIER=tail-<ppid>

### Edge Cases
- Empty input: Handle gracefully
- Very large input: Stream processing line by line
- No stdin: Wait for input or show usage

## Technical Implementation

### Dependencies
- `systemd` crate - Native Rust interface to systemd journal (crates.io)
- `libc` - For getting parent process ID (via std::os::unix::process)
- `std::io` - Read stdin
- `std::env` - Get parent process ID

### Architecture
```
main.rs
├── Read stdin line by line
├── Get PPID using std::os::unix::process::parent_id
├── For each line:
│   ├── Print with tag prefix to stdout
│   └── Send to systemd journal using systemd::journal::print()
```

### Cargo Configuration for GitHub Installation
- Repository: github.com/org/tailing
- Package name in Cargo.toml: `tail`
- Binary name: `tail`
- Install command: `cargo install --git https://github.com/org/tailing --bin tail`

## Implementation Steps
1. Create Cargo.toml with project metadata (name: "tail", bin: "tail")
2. Add `systemd` crate dependency
3. Implement stdin reading (line by line)
4. Get and format PPID as tag
5. Use systemd::journal::print() to log to journal
6. Print with tag prefix
7. Add error handling
8. Add proper CI/Workflow files for GitHub installation

## Acceptance Criteria
- [ ] Reads piped input correctly
- [ ] Generates correct tag format: tail-<ppid>
- [ ] Logs to systemd journal with correct identifier
- [ ] Prints each line with tag prefix
- [ ] Handles empty input gracefully
- [ ] Installable via `cargo install --git https://github.com/org/tailing`

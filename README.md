# tail

A drop-in replacement for the system `tail` command that also logs all output to the systemd journal.

## Installation

```bash
cargo install --git https://github.com/worthies/tailing
```

## Usage

```bash
# Pipe any command output through tail
some_command | tail

# View logs in journalctl
journalctl -f -t tailing
```

## Features

- Drops in as a replacement for `/usr/bin/tail`
- Logs all input lines to systemd journal with identifier `tail-<PPID>`
- Streams input in real-time (line-by-line)
- Prints the tag to stderr for easy identification

## How It Works

1. Reads stdin line by line
2. For each line:
   - Logs to systemd journal via `/usr/bin/logger -t tail-<PPID>`
   - Pipes to `/usr/bin/tail` for standard output
3. Tag is printed to stderr at startup

## License

MIT

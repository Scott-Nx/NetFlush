# NetCount Reset (Rust Implementation)

This is a high-performance, safe implementation of the NetCount Reset utility written in Rust. It interacts directly with the Windows API to manage Network Profile registry keys, eliminating the overhead of batch scripts and external process spawning.

## Features

- **Fast & Efficient:** Direct Windows Registry interaction.
- **Safe:** Handles errors and missing permissions gracefully.
- **Task Scheduler Friendly:** Includes a background mode to run silently without hanging on user input.

## Prerequisites

To build this project, you need the **Rust** toolchain installed.
Get it from [rustup.rs](https://rustup.rs/).

## Build Instructions

1. Open a terminal in this directory (`rust/`).
2. Run the build command:

```powershell
    cargo build --release
```

3. The compiled executable will be located at:
    `target\release\netcount-reset.exe`

## Usage

### 1. Standard Interactive Mode

Run as **Administrator**. This mode will show progress and pause if an error occurs (like missing permissions), ensuring you see the message.

```powershell
# Default (searches for "Network")
netcount-reset.exe

# Custom Profile Name
netcount-reset.exe "MyWifi"
```

### 2. Background Mode (Task Scheduler)

Use the `--background` or `-b` flag. This mode suppresses standard output and **does not pause** on errors, making it safe for automated tasks.

```powershell
# Run silently for "Network" profiles
netcount-reset.exe --background

# Run silently for custom profiles
netcount-reset.exe "MyWifi" --background
```

## Task Scheduler Configuration

When setting up the Windows Task Scheduler action:

- **Program/script:** Path to `netcount-reset.exe`
- **Add arguments (optional):** `--background`

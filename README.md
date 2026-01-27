# NetFlush

Reset Windows “Network” profile numbering by removing old network profile entries from the registry, so you stop seeing names like “Network 2”, “Network 3”, etc. The next time Windows creates a profile, it will start back at “Network”.

## Features

- **Fast & Efficient:** Direct Windows Registry interaction.
- **Safe:** Handles errors and missing permissions gracefully.
- **Task Scheduler Friendly:** Includes a background mode to run silently without hanging on user input.

## Prerequisites

To build this project, you need the **Rust** toolchain installed.
Get it from [rustup.rs](https://rustup.rs/).

## Build Instructions

1. Open a terminal in the project root.
2. Run the build command:

   **Standard Windows (MSVC):**

   ```powershell
   cargo build --release
   ```

   **Cross-compile / GNU Target:**
   If you are compiling from Linux or prefer the GNU toolchain:

   ```bash
   # Add the target (once)
   rustup target add x86_64-pc-windows-gnu

   # Build
   cargo build --release --target x86_64-pc-windows-gnu
   ```

3. The compiled executable will be located at:
   - Standard: `target\release\netflush.exe`
   - GNU: `target\x86_64-pc-windows-gnu\release\netflush.exe`

## Usage

### 1. Standard Interactive Mode

Run as **Administrator**. This mode will show progress and pause if an error occurs (like missing permissions), ensuring you see the message.

```powershell
# Default (searches for "Network")
netflush.exe

# Custom Profile Name
netflush.exe "MyWifi"
```

### 2. Background Mode (Task Scheduler)

Use the `--background` or `-b` flag. This mode suppresses standard output and **does not pause** on errors, making it safe for automated tasks.

```powershell
# Run silently for "Network" profiles
netflush.exe --background

# Run silently for custom profiles
netflush.exe "MyWifi" --background
```

## Task Scheduler Configuration

When setting up the Windows Task Scheduler action:

- **Program/script:** Path to `netflush.exe`
- **Add arguments (optional):** `--background`

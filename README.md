# NetCount-Reset

Reset Windows “Network” profile numbering by removing old network profile entries from the registry, so you stop seeing names like “Network 2”, “Network 3”, etc. The next time Windows creates a profile, it will start back at “Network”.

- Script: `scripts/NetCount Reset.cmd`
- Optional Task Scheduler definition: `tasks/NetCount Reset.xml`
- License: GNU GPLv3 (see `LICENSE`)

Note: This tool modifies the Windows registry. Use responsibly, ideally after exporting a backup of the profiles key.

## What it does

Windows stores network profiles under:
`HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\NetworkList\Profiles`

Over time, profiles accumulate and Windows increments the name (e.g., “Network 5”). The script deletes profile subkeys whose `ProfileName` begins with a chosen prefix (default: `Network`). After removal, Windows forgets those specific profiles and will reuse the base name on next creation.

What gets removed:
- Only registry subkeys in `...NetworkList\Profiles` where `ProfileName` starts with the configured prefix (case-insensitive).
- Other profiles remain intact.

Effect on Windows:
- The system forgets profile-specific settings (e.g., Private/Public selection) for the deleted names.
- The visible network name counter resets.

## How it works

The batch script:
- Enumerates all subkeys under `HKLM\...\NetworkList\Profiles`
- Reads the `ProfileName` value for each
- If the profile name starts with the configured prefix, deletes that subkey
- Runs silently; keys that don’t match are untouched

Implementation details:
- Default prefix is `Network` via the `profileName` variable inside the script
- Must be run elevated to delete from `HKLM`

## Requirements

- Windows 10/11 (or any Windows with the `NetworkList\Profiles` store)
- Administrator privileges to modify `HKLM`
- For the scheduled task import: Task Scheduler permissions to create a task

## Install

You can run the script in place or copy it to a fixed location.

Option A — Run from the repo:
1. Open an elevated Command Prompt (Run as administrator).
2. Navigate to this repository directory.
3. Execute the script: `scripts\NetCount Reset.cmd`

Option B — Install to Program Files (matches the sample task XML):
1. Copy `scripts\NetCount Reset.cmd` to `C:\Program Files\NChalapinyo\NetCount Reset.cmd`
   - Create the `C:\Program Files\NChalapinyo` folder if it doesn’t exist.
2. Ensure the file remains named `NetCount Reset.cmd`.

## Usage

Run on demand:
- Right-click `NetCount Reset.cmd` and choose “Run as administrator”
- Or from an elevated Command Prompt: `"C:\Program Files\NChalapinyo\NetCount Reset.cmd"`

After running:
- Disconnect/reconnect to the network, or reboot
- Windows will recreate a fresh profile without the incremented suffix (e.g., back to “Network”)

## Scheduled Task (optional)

A Task Scheduler definition is provided at `tasks\NetCount Reset.xml`. It’s configured to:
- Run with highest privileges
- Execute: `"C:\Program Files\NChalapinyo\NetCount Reset.cmd"`
- Have no triggers by default (on-demand only). You can add your own triggers.

Import steps:
1. Open Task Scheduler.
2. Action > Import Task…
3. Select `tasks\NetCount Reset.xml`.
4. On the General tab:
   - Change the user to your account (the XML contains a specific SID from the original system).
   - Check “Run with highest privileges”.
5. On the Actions tab:
   - Verify the script path matches where you placed the file.
6. Optionally add a Trigger (e.g., “At log on”, “On workstation unlock”, or a weekly schedule).
7. Save. Run the task on demand to reset profiles as needed.

CLI import (alternative):
- From an elevated prompt in the repository root (or where the XML resides):
  - `schtasks /Create /TN "\NChalapinyo\NetCount Reset" /XML "tasks\NetCount Reset.xml" /F`
- Then, ensure the Action path points to your actual script location.

## Customization

Change which profiles get deleted:
- Open `NetCount Reset.cmd` in a text editor.
- Update the line that sets the prefix: `set "profileName=Network"`
  - Example: `set "profileName=Office"` will remove profiles starting with “Office”.

Tips:
- If your Windows display language uses a different base word (e.g., localized “Network”), set `profileName` to that exact prefix.
- Keep quotes around the value, especially if using spaces.

Advanced (optional):
- The current script uses a fixed prefix variable. If you prefer passing a prefix at runtime, you can enhance the script to read `%1` and fall back to `Network` if not provided.

## Verify it worked

Before running:
- List existing profiles and names to see what will be affected. For PowerShell:
  - `Get-ChildItem "HKLM:\SOFTWARE\Microsoft\Windows NT\CurrentVersion\NetworkList\Profiles" | Get-ItemProperty | Select-Object ProfileName`

After running:
- Check that profiles with the target prefix are gone from the registry path above.
- Connect to a network and confirm the display name no longer increments (e.g., “Network” instead of “Network 6”).

Optional backup:
- Export the registry key before changes:
  - `reg export "HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\NetworkList\Profiles" "%USERPROFILE%\Desktop\Profiles-backup.reg"`

## Troubleshooting

- Nothing changes / Access denied
  - Ensure you’re running elevated (Administrator). Non-elevated runs cannot delete under `HKLM`.

- Wrong profile names aren’t removed
  - Confirm the exact naming and adjust `profileName`. The match is prefix-based and case-insensitive.

## Uninstall

- Delete the scheduled task (if created).
- Remove the script from `C:\Program Files\NChalapinyo\` (or wherever you placed it).

## Safety notes

- This tool deletes specific registry subkeys under `...NetworkList\Profiles` that match your chosen prefix. Always verify the prefix to avoid removing unintended profiles.
- Consider exporting the profiles key before running, so you can restore if needed.

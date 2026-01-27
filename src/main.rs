use std::env;
use std::io;
use std::process;
use winreg::enums::*;
use winreg::RegKey;

fn main() -> io::Result<()> {
    // 1. Parse Arguments
    let args: Vec<String> = env::args().collect();

    // Check for background/silent flag
    let background_mode = args.iter().any(|arg| arg == "--background" || arg == "-b");

    // Filter out the executable path and the background flag to find the profile name
    let clean_args: Vec<&String> = args.iter()
        .skip(1) // Skip executable name
        .filter(|arg| *arg != "--background" && *arg != "-b")
        .collect();

    let profile_prefix = if !clean_args.is_empty() {
        clean_args[0]
    } else {
        "Network"
    };

    if !background_mode {
        println!("Searching for network profiles starting with: \"{}\"...\n", profile_prefix);
    }

    // 2. Open Registry Key: HKLM\...\Profiles
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let profiles_path = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\NetworkList\Profiles";

    // Open with KEY_ALL_ACCESS to allow enumerating AND deleting subkeys.
    // If this fails with PermissionDenied, it means we are not Administrator.
    let profiles_key = match hklm.open_subkey_with_flags(profiles_path, KEY_ALL_ACCESS) {
        Ok(k) => k,
        Err(e) => {
            if e.kind() == io::ErrorKind::PermissionDenied {
                eprintln!("ERROR: Administrator privileges required.");

                if !background_mode {
                    eprintln!("====================================================");
                    eprintln!(" Please right-click and select \"Run as administrator\".");
                    eprintln!("====================================================");
                    // Pause only if NOT in background mode
                    println!("Press Enter to exit...");
                    let _ = std::io::stdin().read_line(&mut String::new());
                }

                process::exit(1);
            }
            // Propagate other errors
            return Err(e);
        }
    };

    // 3. Iterate over subkeys
    // enum_keys() returns an iterator of Results<String> (key names)
    for name_result in profiles_key.enum_keys() {
        if let Ok(name) = name_result {
            // Open the subkey to read its 'ProfileName' value
            if let Ok(subkey) = profiles_key.open_subkey(&name) {
                // Read "ProfileName"
                let profile_name_result: Result<String, _> = subkey.get_value("ProfileName");

                if let Ok(profile_name) = profile_name_result {
                    // Check if it starts with the prefix (Case-Insensitive)
                    if profile_name.to_lowercase().starts_with(&profile_prefix.to_lowercase()) {
                        if !background_mode {
                            println!("Found matching profile: \"{}\"", profile_name);
                        }

                        // 4. Delete the subkey
                        // delete_subkey is called on the PARENT key (profiles_key)
                        match profiles_key.delete_subkey(&name) {
                            Ok(_) => {
                                if !background_mode { println!("  - Deleted successfully."); }
                            },
                            Err(e) => {
                                // Always log errors even in background mode
                                eprintln!("  - Failed to delete \"{}\". Error: {}", profile_name, e);
                            }
                        }
                    }
                }
            }
        }
    }

    if !background_mode {
        println!("\nOperation complete.");
    }
    Ok(())
}

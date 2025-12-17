use std::env;
use std::fs;
use std::io::{self, Write};
use std::os::windows::fs::symlink_dir;
use std::path::PathBuf;
use std::process::Command;

#[cfg(windows)]
fn is_elevated() -> bool {
    use std::ptr;
    use winapi::um::handleapi::CloseHandle;
    use winapi::um::processthreadsapi::{GetCurrentProcess, OpenProcessToken};
    use winapi::um::securitybaseapi::GetTokenInformation;
    use winapi::um::winnt::{TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY};

    unsafe {
        let mut handle = ptr::null_mut();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut handle) == 0 {
            return false;
        }

        let mut elevation = TOKEN_ELEVATION { TokenIsElevated: 0 };
        let mut size = 0;
        let result = GetTokenInformation(
            handle,
            TokenElevation,
            &mut elevation as *mut _ as *mut _,
            std::mem::size_of::<TOKEN_ELEVATION>() as u32,
            &mut size,
        );
        CloseHandle(handle);

        result != 0 && elevation.TokenIsElevated != 0
    }
}

#[cfg(windows)]
fn elevate_self() -> io::Result<()> {
    let exe = env::current_exe()?;
    let dir = env::current_dir()?;

    Command::new("powershell")
        .arg("-Command")
        .arg(format!(
            "Start-Process '{}' -Verb RunAs -WorkingDirectory '{}'",
            exe.display(),
            dir.display()
        ))
        .spawn()?;

    std::process::exit(0);
}

fn main() -> io::Result<()> {
    // Check for admin privileges
    if !is_elevated() {
        println!("Requesting administrator privileges...");
        elevate_self()?;
        return Ok(());
    }

    println!("\n=== iRacing Setup Symlink Creator ===\n");

    // Get user input
    print!("Enter the destination subfolder name (e.g. your BEST SETUPS EVER!!!!): ");
    io::stdout().flush()?;

    let mut user_folder = String::new();
    io::stdin().read_line(&mut user_folder)?;
    let user_folder = user_folder.trim();

    if user_folder.is_empty() {
        eprintln!("Error: No folder name provided.");
        pause();
        return Ok(());
    }

    // Get Documents path
    let documents = match env::var("USERPROFILE") {
        Ok(profile) => PathBuf::from(profile).join("Documents"),
        Err(_) => {
            eprintln!("Error: Could not find Documents folder.");
            pause();
            return Ok(());
        }
    };

    let dest_base = documents.join("iRacing").join("setups");

    // Get current directory (where car folders are)
    let current_dir = env::current_dir()?;

    println!("\nCreating symlinks from {} to {}\n", current_dir.display(), dest_base.display());

    let mut created = 0;
    let mut skipped = 0;
    let mut failed = 0;

    // Iterate through car directories
    if let Ok(entries) = fs::read_dir(&current_dir) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    let car_name = entry.file_name();
                    let car_path = entry.path();

                    // Iterate through track directories
                    if let Ok(track_entries) = fs::read_dir(&car_path) {
                        for track_entry in track_entries.flatten() {
                            if let Ok(track_type) = track_entry.file_type() {
                                if track_type.is_dir() {
                                    let track_name = track_entry.file_name();
                                    let source_path = track_entry.path();
                                    let dest_path = dest_base
                                        .join(&car_name)
                                        .join(user_folder)
                                        .join(&track_name);

                                    // Create parent directory if needed
                                    if let Some(parent) = dest_path.parent() {
                                        fs::create_dir_all(parent).ok();
                                    }

                                    // Create symlink if it doesn't exist
                                    if !dest_path.exists() {
                                        match symlink_dir(&source_path, &dest_path) {
                                            Ok(_) => {
                                                println!("[OK] Created: {}", dest_path.display());
                                                created += 1;
                                            }
                                            Err(e) => {
                                                println!("[FAILED] {}: {}", dest_path.display(), e);
                                                failed += 1;
                                            }
                                        }
                                    } else {
                                        println!("[SKIP] Already exists: {}", dest_path.display());
                                        skipped += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("\n=== Summary ===");
    println!("Created: {}", created);
    println!("Skipped: {}", skipped);
    println!("Failed: {}", failed);
    println!("\nDone!\n");

    pause();
    Ok(())
}

fn pause() {
    print!("Press Enter to exit...");
    io::stdout().flush().ok();
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
}
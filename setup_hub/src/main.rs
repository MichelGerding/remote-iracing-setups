use std::env;
use std::fs;
use std::io::{self, Write};
use std::os::windows::fs::symlink_dir;
use std::process::Command;
use rpassword::read_password;
use reqwest::blocking::Client;

const DOMAIN: &str = "setups.michel-gerding.nl";

fn main() -> io::Result<()> {
    println!("-------------------------------------------------------");
    println!("Setup Hub: Verified Connection Tool");
    println!("-------------------------------------------------------");

    // 1. Ensure WebDAV WebClient Service is active
    let _ = Command::new("sc").args(["start", "WebClient"]).output();

    // 2. Check for existing credentials
    let mut update_needed = true;
    if check_credentials_exist(DOMAIN) {
        print!("\nCredentials for {} already exist. Update them? (y/n) [n]: ", DOMAIN);
        io::stdout().flush()?;
        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        if choice.trim().to_lowercase() != "y" {
            println!("Proceeding with existing credentials...");
            update_needed = false;
        }
    }

    // 3. Prompt for username/password and save in Windows Vault if needed
    if update_needed {
        let (username, password) = get_verified_credentials()?;

        println!("\n[3/6] Saving credentials to Windows Vault...");
        let status = Command::new("cmdkey")
            .args([
                &format!("/add:{}", DOMAIN),
                &format!("/user:{}", username),
                &format!("/pass:{}", password)
            ])
            .status()?;

        if !status.success() {
            return Err(io::Error::other("Failed to save credentials to Windows Vault."));
        }
    }

    // 4. Identify Parent Folder
    let current_dir = env::current_dir()?;
    let parent_name = current_dir
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| io::Error::other("Could not detect folder name"))?;

    println!("\n[4/6] Detected current category: {}", parent_name);

    // 5. Construct UNC WebDAV path directly
    let remote_path = format!(r"\\{}@SSL\DavWWWRoot\mothervault\{}", DOMAIN, parent_name);

    // 5b. Verify that the remote folder actually exists before linking
    println!("[5/6] Checking remote folder: {}", remote_path);

    match fs::read_dir(&remote_path) {
        Ok(_) => {
            println!("Remote folder found. Continuing...");
        }
        Err(_) => {
            return Err(io::Error::other(format!(
                "Remote folder does not exist or is inaccessible: {}",
                remote_path
            )));
        }
    }

    // 6. Create the local symlink
    print!("[5/6] Enter name for local link (default: customs): ");
    io::stdout().flush()?;
    let mut link_input = String::new();
    io::stdin().read_line(&mut link_input)?;
    let mut link_name = link_input.trim();
    if link_name.is_empty() { link_name = "customs"; }

    let target_path = current_dir.join(link_name);

    if target_path.exists() {
        let _ = fs::remove_dir_all(&target_path);
    }

    println!("Creating link: {:?} -> {}", target_path, remote_path);
    match symlink_dir(&remote_path, &target_path) {
        Ok(_) => {
            println!("\nSUCCESS: Setup Hub folder linked!");
        }
        Err(e) => {
            eprintln!("\nERROR: Symlink failed: {}", e);
            eprintln!("Ensure Developer Mode is ON or run as Administrator, and WebClient is running.");
        }
    }

    pause();
    Ok(())
}

// Check if credentials for the domain already exist in Windows Vault
fn check_credentials_exist(domain: &str) -> bool {
    let output = Command::new("cmdkey")
        .args(["/list", domain])
        .output();

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            stdout.contains(domain)
        }
        Err(_) => false,
    }
}

// Prompt for username/password and verify via HTTPS
fn get_verified_credentials() -> io::Result<(String, String)> {
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|e| io::Error::other(e))?;

    loop {
        print!("\n[1/6] Enter Username: ");
        io::stdout().flush()?;
        let mut username = String::new();
        io::stdin().read_line(&mut username)?;
        let username = username.trim().to_string();

        print!("[2/6] Enter Password: ");
        io::stdout().flush()?;
        let password = read_password()?;

        print!("Verifying credentials with {}... ", DOMAIN);
        io::stdout().flush()?;

        let res = client.get(format!("https://{}", DOMAIN))
            .basic_auth(&username, Some(&password))
            .send();

        match res {
            Ok(response) => {
                let status = response.status().as_u16();
                if status < 400 || status == 405 {
                    println!("Verified!");
                    return Ok((username, password));
                } else if status == 401 {
                    println!("\nERROR: Invalid username or password.");
                } else {
                    println!("\nERROR: Server returned status {}", status);
                }
            }
            Err(e) => {
                println!("\nERROR: Could not connect to server: {}", e);
            }
        }
    }
}

// Simple pause before exit
fn pause() {
    print!("\nPress Enter to exit...");
    let _ = io::stdout().flush();
    let mut _u = String::new();
    let _ = io::stdin().read_line(&mut _u);
}

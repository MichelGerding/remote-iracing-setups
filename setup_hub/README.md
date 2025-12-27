# Setup Hub

A utility tool for setting up verified connections to the Setup Hub WebDAV service.

## Purpose

This tool helps users connect to the Setup Hub WebDAV service at setups.michel-gerding.nl for sharing and accessing setups. It handles credential management and verification.

## Features

- Automatic WebDAV service activation
- Credential input and verification
- Secure storage in Windows Vault
- Parent folder identification and symlink creation
- Connection testing

## Requirements

- Windows (uses Windows-specific commands and Vault)
- Administrator privileges (for some operations)

## Usage

Run the tool:

```bash
cargo run
```

The tool will guide you through:

1. Activating the WebDAV service
2. Entering/verifying credentials
3. Saving credentials securely
4. Setting up folder mappings

## What it does

- Starts the WebClient service
- Prompts for username/password if not already stored
- Verifies credentials by attempting a connection
- Saves credentials to Windows Credential Manager
- Creates symbolic links for easy access
- Tests the connection

## Notes

- Credentials are stored securely in the Windows Vault
- The tool requires network access to verify connections
- Run as administrator if symlink creation fails

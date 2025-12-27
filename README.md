# Setup Hub

A collection of tools for managing setup data and connections.

## Components

### downloader_site
A web application that automatically downloads datapacks. It provides a web interface for monitoring downloads and managing admin settings.

### setup_hub
A utility tool for setting up verified connections to the Setup Hub WebDAV service for sharing setups.

## Setup

### Prerequisites
- Rust (latest stable)
- Windows (for setup_hub, due to Windows-specific features)

### Building
```bash
# Build all components
cargo build --release

# Or build individually
cd downloader_site && cargo build --release
cd ../setup_hub && cargo build --release
```

### Running

#### downloader_site
Set the required environment variables:
- `REFRESH_TOKEN`: Your refresh token
- `APEX_AUTH_URL`: Auth endpoint
- `APEX_SIMDATA_URL`: Simdata endpoint
- `APEX_MEMBER_URL`: Member endpoint
- `ADMIN_USERNAME`: Admin username (default: admin)
- `ADMIN_PASSWORD`: Admin password (default: changeme)

Then run:
```bash
cd downloader_site
cargo run
```

Access the web interface at http://localhost:3000
Admin panel at http://localhost:3000/admin

#### setup_hub
Run the setup tool:
```bash
cd setup_hub
cargo run
```

This will guide you through setting up WebDAV credentials for setups.michel-gerding.nl.

## Docker

A docker-compose.yml is provided for easy deployment.

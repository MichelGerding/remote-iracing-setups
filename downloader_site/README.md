# Setup Hub Downloader Site

A web application for automatically downloading datapacks.

## Features

- Automatic JWT token refresh
- Scheduled metadata fetching
- Scheduled file downloads
- Web interface for monitoring
- Admin panel for configuration

## Environment Variables

### Required
- `REFRESH_TOKEN`: Your refresh token

### Optional
- `APEX_AUTH_URL`: Auth endpoint (default: https://auth.apexracinguk.com/auth/refresh-token)
- `APEX_SIMDATA_URL`: Simdata endpoint (default: https://simdata.apexracinguk.com/get-all-metadata)
- `APEX_MEMBER_URL`: Member endpoint base (default: https://member.apexracinguk.com/member)
- `ADMIN_USERNAME`: Admin username (default: admin)
- `ADMIN_PASSWORD`: Admin password (default: changeme)

## Running

1. Set the environment variables
2. Run `cargo run`
3. Access at http://localhost:3000
4. Admin at http://localhost:3000/admin

## API Endpoints

- `/`: Main page
- `/admin`: Admin panel (requires login)
- `/admin/api/jwt`: Get current JWT
- `/admin/api/update-refresh-token`: Update refresh token
- `/admin/api/refresh-jwt`: Refresh JWT
- `/admin/api/download`: Trigger download

## Scheduling

- JWT refresh: Every 50 minutes
- Metadata fetch: Every 2 hours
- File download: Every 2 hours

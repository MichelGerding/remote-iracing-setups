# Michel's Setup Sync

Downloads your setups automatically and serves them via WebDAV so you can access them directly in iRacing.

## Server Setup

### Local Development

1. Create a `setups` folder:
```bash
mkdir setups
```

2. Build and run:
```bash
docker-compose up --build
```

- Info Page: http://localhost:3000
- Admin Panel: http://localhost:3000/admin (default: `admin` / `changeme`)
- WebDAV: http://localhost/dav/ (username: `michel`, password: set in Caddyfile)

### Production (setups.michelgerding.nl)

1. Copy files to server:
```bash
scp docker-compose.yml server:/opt/stacks/setup-sync/
scp Caddyfile.prod server:/opt/stacks/setup-sync/Caddyfile
```

2. Generate password hash on server:
```bash
docker run --rm caddy:2 caddy hash-password --plaintext 'your-password'
```

3. Update the Caddyfile with the hashed password

4. Create `config.json`:
```json
{
  "refresh_token": "YOUR_REFRESH_TOKEN",
  "admin_username": "admin",
  "admin_password": "your-secure-admin-password"
}
```

5. Deploy:
```bash
docker-compose -f docker-compose.yml up -d
```

## Client Setup (Windows - File Explorer Only)

No command line required! Everything done via File Explorer.

### Step 1: Map Network Drive

1. Open **File Explorer** (Windows + E)
2. Right-click **This PC** → **Map network drive...**
3. Choose drive letter **Z:**
4. Enter folder: `https://setups.michelgerding.nl/dav/`
5. Check ✅ **Connect using different credentials**
6. Click **Finish**
7. Enter username `michel` and your password
8. Check ✅ **Remember my credentials**

### Step 2: Create Shortcut in iRacing

1. Navigate to your iRacing setups folder:
   `C:\Users\YOUR_USERNAME\Documents\iRacing\setups\CAR_NAME`
2. Open a second File Explorer window to **Z:** drive
3. Find the matching car folder
4. Right-click the car folder → **Create shortcut**
5. Cut the shortcut and paste it into your iRacing car folder
6. Rename it to `michel-setups`

### Result

```
📁 Documents/iRacing/setups/porsche718gt4mr/
   📁 your-local-setups/
   📁 michel-setups → (shortcut to Z:\porsche718gt4mr)
      📁 spa/
         📄 quali_setup.sto
```

In iRacing, select the `michel-setups` folder to browse remote setups by track.

## Features

- Automatic JWT refresh every 50 minutes
- Automatic file download every 2 hours  
- Only downloads `.sto` setup files
- Skips already downloaded files
- WebDAV access (read-only for clients)
- Basic Auth protected admin panel

## Building Custom Caddy Image

The WebDAV module needs to be compiled into Caddy:

```bash
cd caddy
docker build -t custom-caddy-webdav:latest .
```

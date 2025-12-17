# Apex Racing Setup Stealer

Downloads your Apex Racing setups automatically and serves them via WebDAV so you can access them directly in iRacing.

## Server Setup

### Local Development

```bash
docker-compose up --build
```

- Info Page: http://localhost:3000
- Admin Panel: http://localhost:3000/admin (default: `admin` / `changeme`)
- WebDAV: http://localhost:8080 (username: `apex`, password: `apex`)

### Production (setups.michelgerding.nl)

1. Create `.env` file:
```bash
WEBDAV_PASSWORD=your-secure-password
ACME_EMAIL=your-email@example.com
```

2. Update `config.json`:
```json
{
  "refresh_token": "YOUR_APEX_REFRESH_TOKEN",
  "admin_username": "admin",
  "admin_password": "your-secure-admin-password"
}
```

3. Deploy:
```bash
docker-compose -f docker-compose.prod.yml up --build -d
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
7. Enter username `apex` and your password
8. Check ✅ **Remember my credentials**

### Step 2: Create Shortcut in iRacing

1. Navigate to your iRacing setups folder:
   `C:\Users\YOUR_USERNAME\Documents\iRacing\setups\CAR_NAME`
2. Open a second File Explorer window to **Z:** drive
3. Find the matching car folder
4. Right-click the car folder → **Create shortcut**
5. Cut the shortcut and paste it into your iRacing car folder
6. Rename it to `apex-setups`

### Result

```
📁 Documents/iRacing/setups/porsche718gt4mr/
   📁 your-local-setups/
   📁 apex-setups → (shortcut to Z:\Porsche 718...)
      📁 Circuit de Spa-Francorchamps/
         📄 quali_setup.sto
```

In iRacing, select the `apex-setups` folder to browse remote setups by track.

## Features

- Automatic JWT refresh every 50 minutes
- Automatic file download every 2 hours  
- Only downloads `.sto` setup files
- Skips already downloaded files
- WebDAV access (read-only for clients)
- Basic Auth protected admin panel


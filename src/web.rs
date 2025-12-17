use axum::{
    body::Body,
    extract::State,
    http::{header, Request, StatusCode},
    middleware::{self, Next},
    response::{Html, IntoResponse, Json, Response},
    routing::{get, post},
    Router,
};
use base64::{engine::general_purpose::STANDARD, Engine};
use serde::Deserialize;
use std::sync::Arc;
use crate::downloader::Downloader;

pub fn create_router(downloader: Arc<Downloader>) -> Router {
    let admin_routes = Router::new()
        .route("/", get(admin_page))
        .route("/api/jwt", get(get_jwt))
        .route("/api/update-refresh-token", post(update_refresh_token))
        .route("/api/refresh-jwt", post(refresh_jwt_endpoint))
        .route("/api/download", post(download_files_endpoint))
        .layer(middleware::from_fn_with_state(
            downloader.clone(),
            basic_auth_middleware,
        ))
        .with_state(downloader.clone());

    Router::new()
        .route("/", get(info_page))
        .nest("/admin", admin_routes)
        .with_state(downloader)
}

async fn basic_auth_middleware(
    State(downloader): State<Arc<Downloader>>,
    request: Request<Body>,
    next: Next,
) -> Response {
    let config = downloader.get_config();
    let config_guard = config.read().await;
    let expected_username = &config_guard.admin_username;
    let expected_password = &config_guard.admin_password;

    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    if let Some(auth) = auth_header {
        if auth.starts_with("Basic ") {
            let encoded = &auth[6..];
            if let Ok(decoded) = STANDARD.decode(encoded) {
                if let Ok(credentials) = String::from_utf8(decoded) {
                    let parts: Vec<&str> = credentials.splitn(2, ':').collect();
                    if parts.len() == 2 && parts[0] == expected_username && parts[1] == expected_password {
                        drop(config_guard);
                        return next.run(request).await;
                    }
                }
            }
        }
    }

    Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .header(header::WWW_AUTHENTICATE, "Basic realm=\"Admin Area\"")
        .body(Body::from("Unauthorized"))
        .unwrap()
}

async fn info_page() -> Html<&'static str> {
    Html(r#"
<!DOCTYPE html>
<html>
<head>
    <title>Apex Racing Setups</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif;
            max-width: 900px;
            margin: 50px auto;
            padding: 20px;
            line-height: 1.6;
            color: #333;
        }
        h1 { color: #2c3e50; }
        h2 { color: #34495e; margin-top: 30px; border-bottom: 2px solid #eee; padding-bottom: 10px; }
        h3 { color: #555; margin-top: 25px; }
        .step {
            background: #f8f9fa;
            border-left: 4px solid #3498db;
            padding: 15px;
            margin: 15px 0;
        }
        .step img {
            max-width: 100%;
            margin-top: 10px;
            border: 1px solid #ddd;
            border-radius: 4px;
        }
        code {
            background: #e9ecef;
            padding: 2px 6px;
            border-radius: 3px;
            font-family: 'Consolas', 'Monaco', monospace;
        }
        .url {
            background: #d4edda;
            padding: 10px;
            border-radius: 5px;
            font-family: monospace;
            word-break: break-all;
            user-select: all;
        }
        .warning {
            background: #fff3cd;
            border-left: 4px solid #ffc107;
            padding: 15px;
            margin: 15px 0;
        }
        .info {
            background: #d1ecf1;
            border-left: 4px solid #17a2b8;
            padding: 15px;
            margin: 15px 0;
        }
        .success {
            background: #d4edda;
            border-left: 4px solid #28a745;
            padding: 15px;
            margin: 15px 0;
        }
        .copy-btn {
            background: #6c757d;
            color: white;
            border: none;
            padding: 5px 10px;
            border-radius: 3px;
            cursor: pointer;
            font-size: 12px;
            margin-left: 10px;
        }
        .copy-btn:hover { background: #545b62; }
        .path-box {
            display: flex;
            align-items: center;
            gap: 10px;
            flex-wrap: wrap;
        }
    </style>
</head>
<body>
    <h1>🏎️ Apex Racing Setups</h1>
    <p>Access your Apex Racing setups directly in iRacing using only File Explorer. No software installation required!</p>

    <h2>📁 Step 1: Map Network Drive</h2>

    <div class="step">
        <strong>1.1</strong> Open <strong>File Explorer</strong> (Windows + E)
    </div>

    <div class="step">
        <strong>1.2</strong> Right-click on <strong>This PC</strong> in the left sidebar
    </div>

    <div class="step">
        <strong>1.3</strong> Click <strong>Map network drive...</strong>
    </div>

    <div class="step">
        <strong>1.4</strong> Choose drive letter <strong>Z:</strong> (or any available letter)
    </div>

    <div class="step">
        <strong>1.5</strong> In the Folder field, enter:
        <div class="path-box">
            <div class="url" id="webdav-url">https://setups.michelgerding.nl/dav/</div>
            <button class="copy-btn" onclick="copyToClipboard('webdav-url')">Copy</button>
        </div>
    </div>

    <div class="step">
        <strong>1.6</strong> Check ✅ <strong>Connect using different credentials</strong>
    </div>

    <div class="step">
        <strong>1.7</strong> Click <strong>Finish</strong>
    </div>

    <div class="step">
        <strong>1.8</strong> Enter your credentials when prompted:
        <ul>
            <li><strong>Username:</strong> <code>apex</code></li>
            <li><strong>Password:</strong> <em>(your password)</em></li>
        </ul>
        Check ✅ <strong>Remember my credentials</strong> to stay connected
    </div>

    <div class="success">
        <strong>✅ Done!</strong> You now have a Z: drive with all available setups organized by car and track.
    </div>

    <h2>📂 Step 2: Create Shortcut in iRacing Folder</h2>

    <div class="info">
        <strong>Goal:</strong> Create a shortcut to the remote setups inside your iRacing setups folder, so they appear in iRacing's setup picker.
    </div>

    <div class="step">
        <strong>2.1</strong> Open your iRacing setups folder. Typically located at:
        <div class="path-box">
            <div class="url" id="iracing-path">C:\Users\YOUR_USERNAME\Documents\iRacing\setups</div>
            <button class="copy-btn" onclick="copyToClipboard('iracing-path')">Copy</button>
        </div>
        <small>(Replace YOUR_USERNAME with your Windows username)</small>
    </div>

    <div class="step">
        <strong>2.2</strong> Navigate to the car folder you want setups for (e.g., <code>porsche718gt4mr</code>)
    </div>

    <div class="step">
        <strong>2.3</strong> Open a second File Explorer window and navigate to your <strong>Z: drive</strong>
    </div>

    <div class="step">
        <strong>2.4</strong> Find the matching car folder on Z: (e.g., <code>Porsche 718 Cayman GT4 Clubsport MR</code>)
    </div>

    <div class="step">
        <strong>2.5</strong> <strong>Right-click</strong> on the car folder on Z: and select <strong>Create shortcut</strong>
    </div>

    <div class="step">
        <strong>2.6</strong> <strong>Cut</strong> (Ctrl+X) the shortcut that was created
    </div>

    <div class="step">
        <strong>2.7</strong> <strong>Paste</strong> (Ctrl+V) it into your iRacing car folder
    </div>

    <div class="step">
        <strong>2.8</strong> <strong>Rename</strong> the shortcut to something simple like <code>apex-setups</code>
    </div>

    <div class="success">
        <strong>✅ Done!</strong> Your folder structure now looks like:
        <pre style="background: #f8f9fa; padding: 10px; margin-top: 10px;">
📁 Documents/iRacing/setups/porsche718gt4mr/
   📁 your-local-setups/
   📁 apex-setups → (shortcut to Z:\Porsche 718...)
      📁 Circuit de Spa-Francorchamps/
         📄 quali_setup.sto
         📄 race_setup.sto</pre>
    </div>

    <h2>🎮 Using in iRacing</h2>

    <div class="info">
        When selecting a setup in iRacing:
        <ol>
            <li>Open the garage</li>
            <li>Go to setups</li>
            <li>You'll see the <code>apex-setups</code> folder alongside your own setups</li>
            <li>Open it to browse by track</li>
            <li>Select any setup to load it</li>
        </ol>
    </div>

    <div class="warning">
        <strong>⚠️ Note:</strong> Remote setups are <strong>read-only</strong>. To modify a setup, first load it, then save it with a new name to your local folder.
    </div>

    <h2>🔧 Troubleshooting</h2>

    <div class="step">
        <strong>Can't connect to network drive?</strong>
        <ol>
            <li>Press <strong>Windows + R</strong></li>
            <li>Type <code>services.msc</code> and press Enter</li>
            <li>Find <strong>WebClient</strong> in the list</li>
            <li>Right-click → <strong>Start</strong></li>
            <li>Right-click → <strong>Properties</strong> → Set Startup type to <strong>Automatic</strong></li>
        </ol>
    </div>

    <h2>📋 Car Folder Reference</h2>
    <p>Common car folder names:</p>
    <table style="width:100%; border-collapse: collapse; margin: 15px 0;">
        <tr style="background:#f8f9fa;"><th style="border:1px solid #ddd; padding:8px;">iRacing Folder</th><th style="border:1px solid #ddd; padding:8px;">Network Drive Folder (Z:)</th></tr>
        <tr><td style="border:1px solid #ddd; padding:8px;"><code>porsche718gt4mr</code></td><td style="border:1px solid #ddd; padding:8px;">Porsche 718 Cayman GT4 Clubsport MR</td></tr>
        <tr><td style="border:1px solid #ddd; padding:8px;"><code>lamborghinievogt3</code></td><td style="border:1px solid #ddd; padding:8px;">Lamborghini Huracán GT3 EVO</td></tr>
    </table>
    <p><small>Browse the Z: drive to see all available cars.</small></p>

    <script>
        function copyToClipboard(elementId) {
            const text = document.getElementById(elementId).innerText;
            navigator.clipboard.writeText(text);
        }
    </script>
</body>
</html>
    "#)
}

async fn admin_page() -> Html<&'static str> {
    Html(r#"
<!DOCTYPE html>
<html>
<head>
    <title>Apex Racing Admin</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif;
            max-width: 800px;
            margin: 50px auto;
            padding: 20px;
        }
        .section {
            margin: 20px 0;
            padding: 20px;
            border: 1px solid #ddd;
            border-radius: 5px;
        }
        input[type="text"] {
            width: 100%;
            padding: 10px;
            margin: 10px 0;
            box-sizing: border-box;
        }
        button {
            padding: 10px 20px;
            margin: 5px;
            background-color: #3498db;
            color: white;
            border: none;
            border-radius: 4px;
            cursor: pointer;
        }
        button:hover {
            background-color: #2980b9;
        }
        #status {
            margin-top: 20px;
            padding: 10px;
            border-radius: 4px;
        }
        .success {
            background-color: #d4edda;
            color: #155724;
        }
        .error {
            background-color: #f8d7da;
            color: #721c24;
        }
    </style>
</head>
<body>
    <h1>🔧 Apex Racing Admin</h1>

    <div class="section">
        <h2>JWT Token</h2>
        <input type="text" id="jwt" placeholder="Current JWT Token" readonly>
        <input type="text" id="refreshToken" placeholder="Enter new refresh token">
        <button onclick="updateToken()">Update Refresh Token</button>
    </div>

    <div class="section">
        <h2>Manual Actions</h2>
        <button onclick="downloadFiles()">Download All Files Now</button>
        <button onclick="refreshJwt()">Refresh JWT Token</button>
    </div>

    <div id="status"></div>

    <script>
        async function loadCurrentJwt() {
            try {
                const response = await fetch('/admin/api/jwt');
                const data = await response.json();
                document.getElementById('jwt').value = data.jwt;
            } catch (error) {
                showStatus('Error loading JWT: ' + error.message, 'error');
            }
        }

        async function updateToken() {
            const refreshToken = document.getElementById('refreshToken').value;
            if (!refreshToken) {
                showStatus('Please enter a refresh token', 'error');
                return;
            }

            try {
                const response = await fetch('/admin/api/update-refresh-token', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ refreshToken })
                });

                const data = await response.json();
                if (response.ok) {
                    showStatus(data.message, 'success');
                    loadCurrentJwt();
                } else {
                    showStatus(data.error, 'error');
                }
            } catch (error) {
                showStatus('Error: ' + error.message, 'error');
            }
        }

        async function refreshJwt() {
            try {
                const response = await fetch('/admin/api/refresh-jwt', { method: 'POST' });
                const data = await response.json();

                if (response.ok) {
                    showStatus(data.message, 'success');
                    loadCurrentJwt();
                } else {
                    showStatus(data.error, 'error');
                }
            } catch (error) {
                showStatus('Error: ' + error.message, 'error');
            }
        }

        async function downloadFiles() {
            showStatus('Downloading files...', 'success');
            try {
                const response = await fetch('/admin/api/download', { method: 'POST' });
                const data = await response.json();

                if (response.ok) {
                    showStatus(data.message, 'success');
                } else {
                    showStatus(data.error, 'error');
                }
            } catch (error) {
                showStatus('Error: ' + error.message, 'error');
            }
        }

        function showStatus(message, type) {
            const status = document.getElementById('status');
            status.textContent = message;
            status.className = type;
        }

        loadCurrentJwt();
    </script>
</body>
</html>
    "#)
}

async fn get_jwt(State(downloader): State<Arc<Downloader>>) -> Json<serde_json::Value> {
    let jwt = downloader.get_current_jwt().await;
    Json(serde_json::json!({ "jwt": jwt }))
}

#[derive(Deserialize)]
struct UpdateRefreshTokenRequest {
    #[serde(rename = "refreshToken")]
    refresh_token: String,
}

async fn update_refresh_token(
    State(downloader): State<Arc<Downloader>>,
    Json(payload): Json<UpdateRefreshTokenRequest>,
) -> impl IntoResponse {
    match downloader.update_refresh_token(payload.refresh_token).await {
        Ok(_) => (
            StatusCode::OK,
            Json(serde_json::json!({ "message": "Refresh token updated and JWT refreshed" }))
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": format!("Failed to update token: {}", e) }))
        ),
    }
}

async fn refresh_jwt_endpoint(State(downloader): State<Arc<Downloader>>) -> impl IntoResponse {
    match downloader.refresh_jwt().await {
        Ok(_) => (
            StatusCode::OK,
            Json(serde_json::json!({ "message": "JWT token refreshed successfully" }))
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": format!("Failed to refresh JWT: {}", e) }))
        ),
    }
}

async fn download_files_endpoint(State(downloader): State<Arc<Downloader>>) -> impl IntoResponse {
    match downloader.download_files().await {
        Ok(count) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "message": format!("Successfully downloaded {} new files", count)
            }))
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": format!("Download failed: {}", e) }))
        ),
    }
}

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

// rust
async fn info_page() -> Html<&'static str> {
    Html(r#"
<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8" />
<meta name="viewport" content="width=device-width,initial-scale=1" />
<title>Setup Hub — Quick Guide</title>
<style>
  body{font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,Ubuntu,sans-serif;max-width:900px;margin:36px auto;padding:18px;color:#222}
  h1{color:#0b3b5a} h2{color:#1f6f8b;margin-top:22px;border-bottom:1px solid #e6eef4;padding-bottom:8px}
  .card{background:#fbfdff;border-radius:8px;padding:14px;margin:14px 0;border-left:4px solid #2b7bb9}
  .muted{color:#555;font-size:0.95rem}
  code{background:#eef6fb;padding:2px 6px;border-radius:4px;font-family:monospace}
  .path{display:flex;align-items:center;gap:10px;flex-wrap:wrap}
  .box{background:#fff;padding:8px;border-radius:6px;border:1px solid #e6eef4;font-family:monospace}
  .btn{background:#1f6f8b;color:#fff;border:none;padding:6px 10px;border-radius:5px;cursor:pointer}
  .note{background:#fffbe6;border-left:4px solid #f0ad4e;padding:12px;border-radius:6px}
  pre{background:#f4f7fb;padding:12px;border-radius:6px;overflow:auto}
  small{color:#666}
</style>
</head>
<body>
  <h1>🏎️ Setup Hub — Quick usage</h1>
  <p class="muted">Small helper to verify credentials, store them in the Windows Vault, and create a local link (symlink) to the remote WebDAV folder so iRacing can access shared setups.</p>

  <h2>What it does</h2>
  <div class="card">
    <ul>
      <li>Starts the Windows WebClient service (best-effort).</li>
      <li>Optionally saves credentials to the Windows Vault via <code>cmdkey</code>.</li>
      <li>Verifies credentials against the server at <code>https://setups.michel-gerding.nl</code>.</li>
      <li>Creates a directory symlink inside the current folder pointing to the matching remote category folder.</li>
    </ul>
  </div>

  <h2>How to use</h2>
  <div class="card">
    <ol>
      <li>Open File Explorer and navigate to the iRacing car/category folder you want to link, e.g.:
        <div class="path"><div class="box" id="local">C:\Users\YOUR_USERNAME\Documents\iRacing\setups\porsche718gt4mr</div>
        <button class="btn" onclick="copy('local')">Copy</button></div>
      </li>
      <li>Place or run the compiled <code>setup_hub.exe</code> from that folder (or run it with that folder as the current working directory).</li>
      <li>Follow prompts: enter username/password, allow saving credentials if desired, and choose the local link name (default: <code>apex</code>).</li>
      <li>On success the tool will create the symlink and offer to open it in Explorer.</li>
    </ol>
  </div>

  <h2>Helpfull information</h2>
  <div class="card">
    <p class="muted">Remote UNC path used by the tool:</p>
    <pre>\\setups.michel-gerding.nl@SSL\DavWWWRoot\apex\{category_name}</pre>
  </div>

  <div class="card">
    <h3>Credentials Commands:</h3>
      <p class="muted">
          Add Credentials
      </p>
    <pre>cmdkey /add:{remote UNC path} /user:{username} /pass:{password}</pre>
      <p class="muted">
          List Credentials
      </p>
    <pre>cmdkey /list</pre>
  </div>


  <div class="card">
    <h3>Linking folders:</h3>
      <p class="muted">
          For linking folders symlinks are used. this can be done manually using the mklink command
      </p>
  </div>

  <div class="note">
    <strong>Troubleshooting</strong>
    <ul>
      <li>If symlink creation fails: run the tool as Administrator or enable Developer Mode to allow symlinks.</li>
      <li>If WebDAV access fails: ensure the Windows WebClient service is running (open <code>services.msc</code> → start <strong>WebClient</strong> and set Startup type to <strong>Automatic</strong>).</li>
      <li>If the remote folder is not found: confirm the folder exists on the server and that the local folder name matches the remote category name.</li>
      <li>Remote setups are read-only; to edit, save a copy into your local setups folder.</li>
    </ul>
  </div>

  <h2>Example layout</h2>
  <pre>Documents\iRacing\setups\porsche718gt4mr\
  ├─ your-local-setups\
  └─ apex → (symlink to \\setups.michel-gerding.nl@SSL\DavWWWRoot\apex\porsche718gt4mr)</pre>

  <script>
    function copy(id){
      const txt = document.getElementById(id).innerText;
      navigator.clipboard?.writeText(txt);
      const btn = event?.target;
      if(btn){ const orig = btn.innerText; btn.innerText = 'Copied'; setTimeout(()=>btn.innerText = orig,800); }
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
    <title>Michel's Setups - Admin</title>
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
    <h1>🔧 Michel's Setups - Admin</h1>

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

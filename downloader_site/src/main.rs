mod models;
mod config;
mod api;
mod downloader;
mod web;

use config::Config;
use downloader::Downloader;
use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobScheduler};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("=== Michel's Setup Sync ===\n");

    let config = match Config::load_or_create() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Configuration Error: {}\n", e);
            eprintln!("To get your refresh token:");
            eprintln!("1. Log into the application");
            eprintln!("2. Capture the network traffic during login");
            eprintln!("3. Look for the refresh-token endpoint response");
            eprintln!("4. Copy the 'refreshToken' value to config.json");
            return Err(e);
        }
    };

    let downloader = Arc::new(Downloader::new(config));

    println!("Initializing application...");

    if let Err(e) = downloader.refresh_jwt().await {
        eprintln!("\n❌ Failed to refresh JWT token: {}\n", e);
        eprintln!("Common causes:");
        eprintln!("1. The refresh token in config.json is invalid or expired");
        eprintln!("2. The refresh token format is incorrect");
        eprintln!("3. Network connectivity issues");
        eprintln!("\nPlease obtain a new refresh token from the application and update config.json");
        return Err(e);
    }

    downloader.fetch_metadata().await?;

    let scheduler = JobScheduler::new().await?;

    // Schedule JWT refresh every 50 minutes (to be safe, token expires in 60)
    let downloader_jwt = downloader.clone();
    scheduler.add(
        Job::new_async("0 */50 * * * *", move |_uuid, _l| {
            let d = downloader_jwt.clone();
            Box::pin(async move {
                println!("Running scheduled JWT refresh...");
                if let Err(e) = d.refresh_jwt().await {
                    eprintln!("Failed to refresh JWT: {}", e);
                }
            })
        })?
    ).await?;

    // Schedule file download every 2 hours
    let downloader_download = downloader.clone();
    scheduler.add(
        Job::new_async("0 0 */2 * * *", move |_uuid, _l| {
            let d = downloader_download.clone();
            Box::pin(async move {
                println!("Running scheduled download...");
                if let Err(e) = d.fetch_metadata().await {
                    eprintln!("Failed to fetch metadata: {}", e);
                }
                match d.download_files().await {
                    Ok(count) => println!("Scheduled download completed: {} files", count),
                    Err(e) => eprintln!("Scheduled download failed: {}", e),
                }
            })
        })?
    ).await?;

    scheduler.start().await?;

    let app = web::create_router(downloader);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("\n✓ Server running on http://localhost:3000");
    println!("✓ JWT will refresh automatically every 50 minutes");
    println!("✓ Files will download automatically every 2 hours\n");

    axum::serve(listener, app).await?;

    Ok(())
}
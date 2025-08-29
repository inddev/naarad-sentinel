mod collectors;
mod config;
mod http_client;

use clap::Parser;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;
use std::time::Duration;
use tokio::time;

use collectors::{cpu_temp, disk_free, device_info};
use config::{Config, interactive_setup, load_config};
use http_client::NaaradClient;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Run interactive setup to configure device
    #[arg(long)]
    setup: bool,
    
    /// API key for Naarad service (required for setup)
    #[arg(long)]
    api_key: Option<String>,
    
    /// Override metrics collection interval in seconds
    #[arg(long)]
    interval: Option<u64>,
    
    /// Run in legacy mode (HTTP server only, no pushing)
    #[arg(long)]
    legacy: bool,
    
    /// Development mode (use localhost endpoint)
    #[arg(long)]
    dev: bool,
}

async fn metrics_handler(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut metrics = String::new();

    // CPU temp
    if let Some(temp) = cpu_temp::get_cpu_temp_celsius() {
        metrics.push_str(&format!("device_cpu_temperature_celsius {}\n", temp));
    }

    // Disk free
    if let Some(free) = disk_free::get_disk_free_mb() {
        metrics.push_str(&format!("device_disk_free_mb {}\n", free));
    }

    // Device info
    let info = device_info::get_device_info();
    metrics.push_str(&format!("sentinel_device_info{{os_type=\"{}\", os_version=\"{}\", kernel=\"{}\", arch=\"{}\", hostname=\"{}\"}} 1\n",
        info.os_type, info.os_version, info.kernel_version, info.architecture, info.hostname));

    Ok(Response::new(Body::from(metrics)))
}

async fn run_legacy_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = ([0, 0, 0, 0], 9101).into();

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(metrics_handler))
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("🖥️  Legacy mode: Sentinel running at http://{}/metrics", addr);
    println!("📊 Serving Prometheus metrics on port 9101");

    if let Err(e) = server.await {
        eprintln!("❌ Server error: {}", e);
    }
    
    Ok(())
}

async fn run_monitoring_loop(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let client = NaaradClient::new(config.clone());
    let interval_duration = Duration::from_secs(config.interval);
    
    println!("🚀 Naarad Sentinel started!");
    println!("📡 Device: {} ({})", config.device_name, config.device_id);
    println!("⏱️  Interval: {} seconds", config.interval);
    println!("🎯 Endpoint: {}", config.endpoint);
    println!();
    
    // Also run the legacy HTTP server in the background
    let _legacy_server = tokio::spawn(async {
        let addr = ([0, 0, 0, 0], 9101).into();
        let make_svc = make_service_fn(|_conn| async {
            Ok::<_, Infallible>(service_fn(metrics_handler))
        });
        let server = Server::bind(&addr).serve(make_svc);
        println!("📊 Legacy HTTP server available at http://{}/metrics", addr);
        let _ = server.await;
    });
    
    let mut interval = time::interval(interval_duration);
    
    loop {
        interval.tick().await;
        
        if let Err(e) = client.send_metrics().await {
            println!("❌ Error sending metrics: {}", e);
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    // Handle setup mode
    if args.setup {
        let api_key = match args.api_key {
            Some(key) => key,
            None => {
                eprintln!("❌ Error: --api-key is required when using --setup");
                eprintln!("   Example: ./naarad-sentinel --setup --api-key=naarad_user_xxxxx");
                std::process::exit(1);
            }
        };
        
        match interactive_setup(api_key) {
            Ok(_) => {
                println!("🎉 Setup completed successfully!");
                return Ok(());
            }
            Err(e) => {
                eprintln!("❌ Setup failed: {}", e);
                std::process::exit(1);
            }
        }
    }
    
    // Handle legacy mode
    if args.legacy {
        println!("🔄 Starting in legacy mode (HTTP server only)");
        return run_legacy_server().await;
    }
    
    // Load configuration
    let mut config = match load_config() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("❌ Error loading configuration: {}", e);
            eprintln!("💡 Tip: Run './naarad-sentinel --setup --api-key=YOUR_KEY' first");
            std::process::exit(1);
        }
    };
    
    // Override interval if provided
    if let Some(interval) = args.interval {
        config.interval = interval;
    }
    
    // Override endpoint for development mode
    if args.dev {
        config.endpoint = "http://localhost:5001/api/devices/metrics/ingest".to_string();
        println!("🔧 Development mode: using localhost endpoint");
    }
    
    // Start monitoring
    run_monitoring_loop(config).await
}

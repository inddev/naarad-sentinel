// src/http_client.rs
use hyper::{Body, Client, Method, Request, Uri};
use hyper::header::{AUTHORIZATION, CONTENT_TYPE};
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use serde_json::json;
use crate::config::Config;
use crate::collectors::{cpu_temp, disk_free, device_info};

pub struct NaaradClient {
    client: Client<HttpsConnector<hyper::client::HttpConnector>>,
    config: Config,
}

impl NaaradClient {
    pub fn new(config: Config) -> Self {
        let https = HttpsConnectorBuilder::new()
            .with_webpki_roots()
            .https_or_http()
            .enable_http1()
            .build();
        let client = Client::builder().build::<_, hyper::Body>(https);
        
        Self {
            client,
            config,
        }
    }
    
    pub async fn send_metrics(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Collect current metrics
        let cpu_temp = cpu_temp::get_cpu_temp_celsius();
        let disk_free = disk_free::get_disk_free_mb();
        let device_info = device_info::get_device_info();
        
        // Create timestamp
        let timestamp = chrono::Utc::now().to_rfc3339();
        
        // Build the payload matching the API format
        let payload = json!({
            "device_id": self.config.device_id,
            "device_name": self.config.device_name,
            "device_type": device_info.os_type,
            "timestamp": timestamp,
            "metrics": {
                "device_cpu_temperature_celsius": cpu_temp,
                "device_disk_free_mb": disk_free,
                "sentinel_device_info": {
                    "os_type": device_info.os_type,
                    "os_version": device_info.os_version,
                    "kernel": device_info.kernel_version,
                    "arch": device_info.architecture,
                    "hostname": device_info.hostname
                }
            }
        });
        
        // Parse the endpoint URL
        let uri: Uri = self.config.endpoint.parse()?;
        
        // Create the request
        let req = Request::builder()
            .method(Method::POST)
            .uri(uri)
            .header(CONTENT_TYPE, "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", self.config.api_key))
            .body(Body::from(payload.to_string()))?;
        
        // Send the request
        let resp = self.client.request(req).await?;
        
        // Check response status
        if resp.status().is_success() {
            println!("📡 Metrics sent successfully at {}", timestamp);
        } else {
            let status = resp.status();
            let body_bytes = hyper::body::to_bytes(resp.into_body()).await?;
            let body_str = String::from_utf8_lossy(&body_bytes);
            
            println!("❌ Failed to send metrics: {} - {}", status, body_str);
        }
        
        Ok(())
    }
}

mod collectors;

use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;
use collectors::{cpu_temp, disk_free, device_info};

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

#[tokio::main]
async fn main() {
    let addr = ([0, 0, 0, 0], 9101).into();

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(metrics_handler))
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Sentinel running at http://{}/metrics", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

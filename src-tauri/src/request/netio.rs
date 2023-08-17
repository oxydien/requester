/* NetIO ( Network - Input/Output ) -> Tcp/Udp */
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::net::ToSocketAddrs;
use std::time::Instant;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, UdpSocket};

// Struct to hold information about the request
#[derive(Debug, Serialize, Deserialize)]
struct RequestInfo {
    protocol: String,
    host: String,
    port: u16,
    data: Vec<u8>,
}

// Struct to hold information about the response
#[derive(Debug, Serialize, Deserialize)]
struct ResponseInfo {
    data: Vec<u8>,
}

// Struct to hold the complete result information
#[derive(Debug, Serialize, Deserialize)]
struct ResultInfo {
    request: RequestInfo,
    response: ResponseInfo,
    time: u128, // in milliseconds
}

// Customization options for the request
pub struct RequestOptions {
    pub protocol: String,
    pub host: String,
    pub port: u16,
    pub data: Vec<u8>,
}

pub async fn create_request(options: RequestOptions) -> Result<String, String> {
    let addr = format!("{}:{}", options.host, options.port);
    let socket_addr = addr
        .to_socket_addrs()
        .map_err(|e| e.to_string())?
        .next()
        .ok_or("Invalid address")?;
    let start_time = Instant::now();

    let (response_data, _protocol) = match options.protocol.as_str() {
        "tcp" => {
            let mut stream = TcpStream::connect(&socket_addr)
                .await
                .map_err(|e| e.to_string())?;
            stream
                .write_all(&options.data)
                .await
                .map_err(|e| e.to_string())?;
            let mut response = Vec::new();
            stream
                .read_to_end(&mut response)
                .await
                .map_err(|e| e.to_string())?;
            (response, "tcp")
        }
        "udp" => {
            let socket = UdpSocket::bind("0.0.0.0:0")
                .await
                .map_err(|e| e.to_string())?;
            socket
                .send_to(&options.data, &socket_addr)
                .await
                .map_err(|e| e.to_string())?;
            let mut response = vec![0u8; 1024];
            let (recv_len, _) = socket
                .recv_from(&mut response)
                .await
                .map_err(|e| e.to_string())?;
            response.truncate(recv_len);
            (response, "udp")
        }
        _ => return Err("Unsupported protocol".into()),
    };

    let elapsed_time = start_time.elapsed().as_millis();

    let result_info = ResultInfo {
        request: RequestInfo {
            protocol: options.protocol.to_string(),
            host: options.host.clone(),
            port: options.port,
            data: options.data.clone(),
        },
        response: ResponseInfo {
            data: response_data,
        },
        time: elapsed_time,
    };

    let result_json = json!(&result_info).to_string();

    Ok(result_json)
}

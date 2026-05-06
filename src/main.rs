// set up basic TCP server
use anyhow;
use serde::Deserialize;
use std::fmt;
use std::fs::File;
use std::io::{Read, Write};
use std::net::SocketAddr;
use std::net::{TcpListener, TcpStream};
use tokio;
use futures::future;

// allow main to be async with tokio::main
#[tokio::main]
async fn main() {
    let servers_with_status: Vec<Server> = check_health().await.expect("unable to do async health check on the servers"); // this is why main needs to be async...
    println!("{servers_with_status:?}")
}

// check connection health for provided servers.
async fn check_health() -> Result<Vec<Server>, anyhow::Error> {
    let server_names: Vec<Server> = get_servers_from_yaml()?;
    let servers = future::join_all(server_names
        .into_iter()
        .map(|mut server| async {
            let socket_addr: SocketAddr = server.name.parse().expect("invalid address");
            match tokio::net::TcpStream::connect(&socket_addr).await {
                Ok(_) => server.can_connect = true,
                Err(e) if e.kind() == std::io::ErrorKind::ConnectionRefused => {
                    server.can_connect = false
                }
                Err(_) => server.can_connect = false,
            }
            server
        })).await;
    Ok(servers)
}

// Only shows current available servers.
async fn available_servers() -> Result<Vec<Server>, anyhow::Error> {
    let server_names: Vec<Server> = get_servers_from_yaml()?;
    let servers: Vec<Server> = future::join_all(server_names
        .into_iter()
        .map(|mut server| async move {
            let socket_addr: SocketAddr = server.name.parse().expect("invalid address");
            if tokio::net::TcpStream::connect(&socket_addr).await.is_ok() {
                    server.can_connect = true;
                    Some(server)
            } else {
                    None
                }
        })).await
        .into_iter()
        .flatten() // only keep Some()
        .collect();
    Ok(servers)
}

// server state tracking: ip:port, up? can we connect to it?, number of connections
// startup behaviour: read yaml, check server is up (ping), start routing traffic

#[derive(Debug, Deserialize, Default)]
struct Server {
    name: String,
    connections: u32,
    can_connect: bool,
}

fn get_servers_from_yaml() -> Result<Vec<Server>, anyhow::Error> {
    // read yaml
    let file: File = File::open("./src/default.yml")?;

    // create vector of servers?
    let server_names: Vec<String> = serde_yaml::from_reader(file)?;
    println!("{server_names:?}");
    let servers: Vec<Server> = server_names
        .into_iter()
        .map(|name| Server {
            name,
            ..Default::default()
        })
        .collect();
    Ok(servers)
}

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

// TODO use this once we're ready to start receiving connections.
fn start_server() -> Result<(), anyhow::Error> {
    let listener = TcpListener::bind("127.0.0.1:43000")?;
    println!("Server listening on 127.0.0.1:43000");

    // Accept connections and handle them
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr()?);
                handle_client(stream)?;
            }
            Err(e) => eprintln!("Error accepting connection: {}", e),
        }
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 512];

    // Read data from the client
    let bytes_read = stream.read(&mut buffer)?;

    // 0 bytes is disconnect
    if bytes_read == 0 {
        // Do nothing?
        return Ok(());
    }

    // Send the same data back to the client (Echo)
    stream.write_all(&buffer[..bytes_read])?;
    stream.flush()?;

    Ok(())
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
            name: name,
            ..Default::default()
        })
        .collect();
    Ok(servers)
}

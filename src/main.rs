// set up basic TCP server
use anyhow;
use serde::Deserialize;
use std::error;
use std::fmt;
use std::fs::File;
use std::io::{Read, Write};
use std::net::TcpStream;
// use std::net::{TcpListener, TcpStream};

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

fn main() {
    /*
        // Bind to localhost:8080
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
    */
    get_servers().unwrap();
}

// get list of available servers
// via yaml file
// server state tracking: ip:port, up? can we connect to it?, number of connections

#[derive(Debug, Deserialize, Default)]
struct Server {
    name: String,
    connections: u32,
    can_connect: bool,
}

fn get_servers() -> Result<Vec<Server>, anyhow::Error> {
    // read yaml
    let file: File = File::open("./src/default.yml")?;

    // create vector of servers?
    let server_names: Vec<String> = serde_yaml::from_reader(file)?;
    println!("{server_names:?}");
    let servers: Vec<Server> = server_names
        .into_iter()
        .map(|name| Server {
            name: name.to_string(),
            ..Default::default()
        })
        .collect();
    Ok(servers)
}

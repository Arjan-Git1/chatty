use std::env;
use anyhow::Result;
use rcgen::generate_simple_self_signed;
use rustls::{Certificate as RustlsCertificate, PrivateKey as RustlsPrivateKey};

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage:");
        println!("Chatty server <ip> <port>");
        println!("Chatty connect <ip> <port>");
        return Ok(());
    }

    if &args[1] == "server" {
        if args.len() < 4 {
            println!("Missing IP or port for server.");
            return Ok(());
        }
        let ip = &args[2];
        let port: u16 = args[3].parse()?;
        server(ip, port).await?;
    } else if &args[1] == "connect" {
        if args.len() < 4 {
            println!("Missing IP or port for client.");
            return Ok(());
        }
        let ip = &args[2];
        let port: u16 = args[3].parse()?;
        connect(ip, port).await?;
    } else {
        println!("Command not recognized.");
    }

    Ok(())
}

async fn server(ip: &str, port: u16) -> Result<()> {
    println!("Starting server on {}:{} ...", ip, port);

    
    let cert = generate_simple_self_signed(vec![ip.to_string()])?;

    
    let cert_der = cert.serialize_der()?;                  
    let key_der = cert.serialize_private_key_der();        

    
    let rustls_cert = RustlsCertificate(cert_der);
    let rustls_key = RustlsPrivateKey(key_der);

    println!("Server certificate and key generated.");
    println!("Certificate bytes: {}, Key bytes: {}", rustls_cert.0.len(), rustls_key.0.len());
    let mut server_config = quinn::ServerConfig::with_single_cert(
        vec![rustls_cert],
        rustls_key
    );
    let endpoint = quinn::Endpoint::server(server_config,format("{}:{}",ip,port).parse()?)?;
    while let Some(conn)= endpoint.accept().await{
        tokio::spawn(handle_connection(conn));
    }
    async fn handle_connection(conn : quinn::Connecting)->anyhow::Result<()>{
        let connection = conn.await?;
        println("Client Found and Connected!: {:?}", connection.remote_adress());
        while let Ok(Some(stream)) = connection.accept_uni().await(){
            tokio::spawn(handle_stream(stream));
        }
    }
    async fn handle_stream(mut stream: quinn::RecvStream)-> anyhow::Result<()>{
        let mut buf = [0u8; 1024];
        while let Ok(n)= stream.read(&mut buf).await{
            println!("Received : {}", String::from_utf8_lossy(&buf[..n]));
        }
    }
    Ok(())
}

async fn connect(ip: &str, port: u16) -> Result<()> {
    println!("Client connecting to {}:{} ...", ip, port);

    

    Ok(())
}

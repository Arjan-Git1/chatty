use std::env;
use quinn::crypto::rustls;
use rcgen::generate_simple_self_signed;
use rcgen::CertifiedKey;
#[tokio::main]
async fn main()-> anyhow::Result<()> {
    let arg : Vec<String> = env.arg().collect();
    if args.len()<2
{
    println!("Usage : ");
    println!("Chatty server <port>");
    println!("Chatty connect <ip> <port>");
}

if &args[1]=="server" {
    let port:u16 = &args[2];
    server(port).await?;
}
else if &args[1]=="connect" {
    let ip:str = &args[2];
    let port:u16 = &args[3];
    connect(ip,port).await?;
}
else{
    println!("Command not recognized.")
}
async fn server(port : u16) -> anyhow::Result<()>{
    //implement the server
    let server_ip = 192.168.0.4;
    let cert = generate_simple_self_signed(vec!["server_ip".to_string()])?;
    let cer_der = cert.serialize_der();
    let key_der = cert.serialise_private_key_der();
    let der_cert = vec![rustls::Certificate(cert_der)];
    let key = rustls::PrivateKey(key_der);
    OK(())
}
async fn connect(ip : &str, port :u16)->anyhow::Result<()>{
    //implement the client side
    // Ok(())
}

}


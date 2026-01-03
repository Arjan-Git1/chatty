use std::env
fn main() {
    let arg : Vec<String> = env.arg().collect();
    if args.len()<2
{
    println!("Usage : ");
    println!("Chatty server <port>");
    println!("Chatty connect <ip> <port>");
}
if &args[1]=="server" {
    
}
else if &args[1]=="connect" {
    
}
else{
    println!("Command not recognized.")
}
async fn server(port : u16) -> anyhow::Result<()>{
    //implement the server
    OK(())
}
async fn connect(ip : &str, port :u16)->anyhow::Result<()>{
    //implement the client side
    // Ok(())
}

}


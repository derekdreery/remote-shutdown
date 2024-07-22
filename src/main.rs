use local_ip_address::local_ip;
use shutdown::shutdown;
use std::net::TcpListener;

type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;

fn main() -> Result {
    let local_addr = local_ip()?;
    let listener = TcpListener::bind("0.0.0.0:8246")?;
    println!("To shutdown connect to {}:8246", local_addr);
    listener.accept()?;
    shutdown()?;
    Ok(())
}

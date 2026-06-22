use std::net::SocketAddr;

mod proxy;
mod pool;
mod health;

fn main() {
    let listen: SocketAddr = "0.0.0.0:8080".parse().unwrap();
    let backends = vec![
        "10.0.1.10:3000".parse().unwrap(),
        "10.0.1.11:3000".parse().unwrap(),
    ];
    
    println!("TCP Proxy listening on {}", listen);
    println!("Backends: {:?}", backends);
}

use std::net::{UdpSocket, Ipv4Addr, SocketAddr, IpAddr};
use std::time::{SystemTime, Duration};

fn main() -> std::io::Result<()> {
    udp_blaster_live::send("127.0.0.1:34254");
    Ok(())
}
use std::net::UdpSocket;
use std::time::SystemTime;

pub fn send(target: &Str) {
    let mut socket = UdpSocket::bind(target).expect("Failed to bind to socket");
    let send = [0x30, 0x31, 0x32, 0x33, 0x34, 0x0a];
    let now = SystemTime::now();
    while now.elapsed().expect("Couldn't get duration").as_millis() <= 5000 {
        socket.send_to(&send, target);
    }
}

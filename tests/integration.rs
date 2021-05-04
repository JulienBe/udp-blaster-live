use std::net::UdpSocket;
use std::time::{Duration, SystemTime};

#[test]
fn perf_test() {
    let mut socket = UdpSocket::bind("127.0.0.1:34254").expect("Failed to bind to socket");
    socket.set_read_timeout(Some(Duration::new(1, 0)));

    let mut receivedCpt: u32 = 0;
    let mut buf = [0; 10];

    udp_blaster_live::send("127.0.0.1:");
    socket.recv_from(&mut buf);
    let start = SystemTime::now();
    while true {
        receivedCpt += 1;
        socket.recv_from(&mut buf);
    }
    let duration = SystemTime::now().duration_since(start).expect("Couldn't get duration").as_millis();
    println!("Received {} packets in {} ms. Throughput: {} packets/ms", receivedCpt, duration, receivedCpt as f32 / duration as f32);
}
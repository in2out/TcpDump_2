use ui;
use net::PacketInfo;
use pcap::Device;
use std::io::{self,Write};

fn main(){
    let mut cap = Device::lookup().unwrap().open().unwrap();
//    while let Ok(packet) = cap.next() {
    for _ in 0..5 {
// ex, 16:55:00.511963 IP SNIPER.ssh > 10.0.80.108.60371: Flags [P.], seq 933769480:933769676, ack 2243359901, win 254, length 196
        let packet = cap.next();
        let mut pInfo = PacketInfo::new();
        match packet {
            Ok(packet) => {
                print!("{}.{} ", packet.header.ts.tv_sec, packet.header.ts.tv_usec);

                pInfo = pInfo.parsing(packet.data); 
                pInfo.print();

                io::stdout().flush().unwrap();
                println!("");
            },
            Err(e) => {
                println!("{:?}", e);
                continue; 
            }
        }
    }

    let stats = cap.stats().unwrap();
    println!("Received: {}, Dropped: {}, if_dropped: {}", stats.received, stats.dropped, stats.if_dropped);
}

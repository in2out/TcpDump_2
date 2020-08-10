//use ui;
use net::PacketInfo;
use net::PacketInfoErr;
use pcap::Device;
use std::io::{self,Write};

fn main(){
    let mut cap = Device::lookup().unwrap().open().unwrap();
//    while let Ok(packet) = cap.next() {
    for _ in 0..5 {
// ex, 16:55:00.511963 IP SNIPER.ssh > 10.0.80.108.60371: Flags [P.], seq 933769480:933769676, ack 2243359901, win 254, length 196
        let packet = cap.next();
        let pkt_info = PacketInfo::new();
        match packet {
            Ok(packet) => {
                print!("{}.{} ", packet.header.ts.tv_sec, packet.header.ts.tv_usec);

                match pkt_info.parsing(packet.data) {
                    Ok(pkt_info) => pkt_info.print(),
                    Err(e) => match e {
                        PacketInfoErr::SizeErrEther => {
                            print!("too small size.");
                        },
                        PacketInfoErr::ConvertErrEther => {
                            print!("error occurred in ether-data");
                        },
                    }
                };

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

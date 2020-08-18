#[cfg(test)]
use net::PacketInfo;
#[cfg(test)]
//use net::PacketInfoErr;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_ether() {
        let data = [0x00, 0x00, 0x05, 0x47, 0x02, 0x90, 0xc4, 0x6c, 0x9c, 0xed, 0xba, 0x65, 0xc0, 0x08, 0x00];

        let pkt_info = PacketInfo::new();
        assert_eq!(pkt_info.parsing(&data).is_err(), true);

        let data = [0x00, 0x00, 0x05, 0x47, 0x02, 0x90, 0xc4, 0x6c, 
                    0x9c, 0xed, 0xba, 0x65, 0x00, 0x08, 0x00, 0x00];
        let pkt_info = PacketInfo::new();
        let pkt_info = match pkt_info.parsing(&data) {
            Ok(pkt_info) => pkt_info,
            Err(_) => { 
                return assert!(false);
            }
        };

        assert_eq!(pkt_info.ip_ver, 4);
    }
}

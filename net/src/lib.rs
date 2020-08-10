use core::convert::TryInto;
//use std::fmt::Debug::fmt;

pub enum PacketInfoErr {
    SizeErrEther,
    ConvertErrEther,
}


#[allow(dead_code)]
pub struct PacketInfo {
    caplen: u32,
    ip_ver: u32, 
    protocol: u32,
    srcip: u32,
    dstip: u32,
    sport: u16,
    dport: u16,
    tcp_flags: u8,
    tcp_seq: u32,
    tcp_ack: u32,
    win: u32,
}

/*
pub struct net_ip_hdr {
    ip_v_hl: u8,      /* version,  header length */
    ip_tos: u8,       /* type of service */
    ip_len: u16,       /* total length */
    ip_id: u16,        /* identification */
    ip_off: u16,       /* fragment, offset */
    ip_ttl: u8,      /* time to live */
    ip_proto: u8,    /* protocol */
    ip_sum: u16,      /* checksum */
    ip_src: u32,      /* source ip address */
    ip_dst: u32,      /* destination ip address */
}
*/

impl PacketInfo {
    pub fn new() -> PacketInfo {
        PacketInfo {
            caplen: 0,
            ip_ver: 0,
            protocol: 0,
            srcip: 0,
            dstip: 0,
            sport: 0,
            dport: 0,
            tcp_flags: 0,
            tcp_seq: 0,
            tcp_ack: 0,
            win: 0,
        }
    }

    pub fn parsing(mut self, data: &[u8]) -> Result<PacketInfo, PacketInfoErr> {
        if data.len() < 16 {
            return Err(PacketInfoErr::SizeErrEther);
        }

        let ether= match data[12..14].try_into() {
            Ok(ether) => ether,
            Err(_) => return Err(PacketInfoErr::ConvertErrEther),
        };

        let ether = u16::from_be_bytes(ether);

        match ether {
            0x0800 => self.ip_ver = 4,
            0x86dd => self.ip_ver = 6,
            _ => self.ip_ver = 0,
        }
        Ok(self)
    }

    pub fn print(self) {
        print!("{} {}.{} > {}.{}, length {}", self.ip_ver, self.srcip, self.sport, self.dstip, self.dport, self.caplen);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

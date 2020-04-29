use std::{io::Read, mem::MaybeUninit};
use packet_crafter::Packet;

fn main() {
    let mut config = tun::Configuration::default();
    config.address((192, 168, 0, 1)).netmask((255, 255, 255, 0)).up();
    config.platform(|c| {
        c.packet_information(true);
    });

    let mut dev = tun::create(&config).expect("Failed to create device");
    let mut buf : [u8; 1504] = unsafe {
        [MaybeUninit::uninit().assume_init(); 1504]
    };

    loop {
        let nbytes = dev.read(&mut buf).unwrap();
        let _eth_flags = u16::from_be_bytes([buf[0], buf[1]]);
        let eth_proto = u16::from_be_bytes([buf[2], buf[3]]);
        if eth_proto != 0x0800 {
            println!("Ignore {:02x} proto", eth_proto);
            continue;
        }
        let packet = Packet::parse(&buf[4..nbytes]).unwrap();
        let tcp_header = packet.get_tcp_header().unwrap();
        println!("{}", tcp_header.get_dst_port());
        println!("receiving {:?}", &buf[..nbytes]);
    }
}

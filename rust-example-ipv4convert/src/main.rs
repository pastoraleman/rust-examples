
use std::net::Ipv4Addr;

fn main() {

    let my_addr = Ipv4Addr::new(192, 168, 20, 1);
    let addru32 = u32::from(my_addr);
    println!("192.168.20.1 = {}", addru32);

    let new_addr = Ipv4Addr::from(168430179);
    println!("168430179 = {}", new_addr);

}

// fn ipv4_to_u32(ipv4: Ipv4Addr) -> u32 {
//
//     let addr_octets = ipv4.octets();
//
//     ((addr_octets[0] as u32) << 24) |
//         ((addr_octets[1] as u32) << 16) |
//         ((addr_octets[2] as u32) << 8) |
//         (addr_octets[3] as u32)
//
// }
//
// fn u32_to_ipv4(ip_u32: u32) -> Ipv4Addr {
//
//     Ipv4Addr::new( (ip_u32 >> 24) as u8,
//                     (ip_u32 >> 16) as u8,
//                     (ip_u32 >> 8) as u8,
//                     ip_u32 as u8 )
//
// }

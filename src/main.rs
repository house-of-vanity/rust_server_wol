use wake_on_lan;
use std::i64;
use std::env;
//use eui48::MacAddress;

fn parse_mac(mac_str: &str) -> [u8; 6] {
  let v: [u8; 6] = mac_str.split(':')
    .into_iter()
    .map(|f| i64::from_str_radix(f, 16).to_owned());

  println!("{:?}", v);
//let mut mac_bytes = Vec::new();
//for i in 0..5 {
//    &mac_bytes.push(i)
//}
  v
}

fn main() {
//  let line: &str = "e8:6a:64:48:b2:bc";

//  let mut address: MacAddress;
//  match MacAddress::parse_str(line) {
//      Err(w) => panic!("{:?}", w),
//      Ok(a) => address = a,
//  }
//  let mac_address: [u8; 6] = [0x0F, 0x1E, 0x2D, 0x3C, 0x4B, 0x5A];
//  let magic_packet = wake_on_lan::MagicPacket::new(&mac_address);
//  magic_packet.send();

//  println!("{:02X?}", mac_address);
//  println!("{:02X?}", address);
    let args: Vec<String> = env::args().collect();
    match args.len() {
          1 => {
              panic!("ALLAH");
          },
          2 => {
              println!("{:?}", parse_mac(&args[1]));
          },
          _ => {
              panic!("ALLAH");
          }
    }
}

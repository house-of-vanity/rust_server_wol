use std::env;
use std::i64;
use wake_on_lan;
use std::process;


fn parse_mac(mac_str: &str) -> [u8; 6] {
    let v: Vec<_> = mac_str
        .split(':')
        .into_iter()
        .map(|f| i64::from_str_radix(f, 16).to_owned())
        .filter_map(Result::ok)
        .collect();
    let mut mac_bytes: [u8; 6] = [0 as u8; 6];
    for i in 0..6 {
        mac_bytes[i] = v[i] as u8;
    }
    mac_bytes
}

fn send_wol(address: &[u8; 6]) {
    let magic_packet = wake_on_lan::MagicPacket::new(&address);
    match magic_packet.send() {
        Ok(ok) => println!("Woked up {:X?}", address),
        Err(err) => println!("Can't wake up {:X?}. {:?}", address, err),
        _ => { panic!("Can't wake up {:X?}.");},
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            println!("Specify a MAC address first.");
process::exit(0x1);
        }
        2 => {
            println!("Got:\t{}", args[1]);
            let address = parse_mac(&args[1]);
            println!("Parsed:\t {:#X?}", address);
            send_wol(&address);
        }
        _ => {
            println!("Unrecognozed options. Specify a MAC address only.");
process::exit(0x1);
        }
    }
}

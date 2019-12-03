use std::env;
use std::i64;
use std::process;
use wake_on_lan;

extern crate router_os;

use router_os::ApiRos;
use std::net::TcpStream;
use std::io::BufRead;
use std::io;

fn get_line() -> String {
   let stdin_u = io::stdin();
   let mut stdin = stdin_u.lock();
   let mut line = String::new();
   stdin.read_line(&mut line).unwrap();
	line.pop();
   return line;
}


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
        Ok(ok) => {
          print!("Woked up ");
          for i in 0..address.len() {
            print!("{:X}{}", address[i], (if i == address.len()-1 { "" } else { ":" }))
          }
          println!("");}
        Err(err) => println!("Can't wake up {:X?}. {:?}", address, err),
        _ => panic!("Can't wake up {:X?}."),
    }
}

fn main() {
    let mut stream = TcpStream::connect("192.168.88.1:8728").unwrap();
    let mut apiros = ApiRos::new(&mut stream);
	apiros.login("alexandr".to_string(), "Tail-fpwd1337".to_string());
    let x = apiros.talk(vec!["/ip/dhcp-server/lease/print".to_string()]);
    println!("{}", x.len());
    for i in 0..x.len()-1 {
        println!("{:?}", x[i].1["=mac-address"]);
    }
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            println!("Specify a MAC address first.");
            process::exit(0x1);
        }
        _ => {
            println!("Got {} addresses.", args.len() - 1);
            for i in 1..args.len() {
                let address = parse_mac(&args[i]);
                send_wol(&address);
            }
        }
    }
}

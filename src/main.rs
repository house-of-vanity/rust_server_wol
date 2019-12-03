use router_os::ApiRos;
use std::{env, fs::File, i64, io, io::prelude::*, io::BufRead, net::TcpStream, process::exit};
use wake_on_lan;

use yaml_rust::yaml::{Hash, Yaml};
use yaml_rust::YamlLoader;

extern crate router_os;

fn get_line() -> String {
    let stdin_u = io::stdin();
    let mut stdin = stdin_u.lock();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    line.pop();
    return line;
}

fn read_config(file: &str) -> Vec<yaml_rust::yaml::Yaml> {
    match File::open(file) {
        Ok(mut file) => {
            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                Ok(_r) => YamlLoader::load_from_str(&contents).unwrap(),
                Err(_r) => {
                    println!("Check config.yaml. Parsing error.");
                    Vec::new()
                }
            }
        }
        Err(_err) => {
            println!("Create config.yaml first");
            Vec::new()
        }
    }
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
                print!(
                    "{:X}{}",
                    address[i],
                    (if i == address.len() - 1 { "" } else { ":" })
                )
            }
            println!("");
        }
        Err(err) => println!("Can't wake up {:X?}. {:?}", address, err),
        _ => panic!("Can't wake up {:X?}."),
    }
}

fn main() {
    let config = read_config("config.yaml");
    let mut stream = TcpStream::connect(config[0]["router"]["addr"].as_str().unwrap()).unwrap();
    let mut apiros = ApiRos::new(&mut stream);
    match apiros.login(
        config[0]["router"]["user"].as_str().unwrap().to_string(),
        config[0]["router"]["pass"].as_str().unwrap().to_string(),
    ) {
        Ok(_u) => {}
        Err(err) => {
            println!("RouterOS authentication error: {:?}", err);
            exit(0x2)
        }
    };
    let x = match apiros.talk(vec!["/ip/dhcp-server/lease/print".to_string()]) {
        Ok(reply) => reply,
        Err(err) => {
            println!("RouterOS API Error: {:?}", err);
            exit(0x1)
        }
    };
    for i in 0..x.len() - 1 {
        println!("{:?}", x[i].1["=mac-address"]);
    }
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            println!("Specify a MAC address first.");
            exit(0x1);
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

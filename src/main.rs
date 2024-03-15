mod config;
mod wol;

fn main() {
    let (name, mac_address) = config::get_mac_to_boot();
    println!("wol {} MAC address: {}", name, mac_address);
    wol::send_wol(mac_address.parse().unwrap(), None, None).unwrap();
}

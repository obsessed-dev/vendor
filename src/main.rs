use std::{env, process};

include!(concat!(env!("OUT_DIR"), "/vendors.rs"));

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();

    if args.len() != 1 {
        println!("Usage: vendor <MAC ADDRESS>...");
        process::exit(1);
    }

    let mac_addr = &args[0];
    let mac_oui = mac_addr.split(':').take(3).collect::<Vec<_>>().join(":");

    match MAC_VENDORS.get(&mac_oui) {
        Some(vendor) => println!("{}", vendor),
        None => println!("Unknown vendor"),
    }
}

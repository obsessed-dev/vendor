use std::{env, process};

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();

    if args.len() != 1 {
        println!("Usage: vendor <MAC ADDRESS>...");
        process::exit(1);
    }

    let mac_addr = &args[0];
    match mac_vendor_lookup::lookup_mac_vendor(&mac_addr) {
        Some(vendor) => println!("{}", vendor),
        None => println!("Unknown vendor"),
    }
}

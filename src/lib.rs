include!(concat!(env!("OUT_DIR"), "/vendors.rs"));

pub fn lookup_mac_vendor(mac_addr: &str) -> Option<&'static str> {
    let mac_oui = mac_addr
        .to_uppercase()
        .split(':')
        .take(3)
        .collect::<Vec<_>>()
        .join(":");

    MAC_VENDORS.get(&mac_oui).copied()
}

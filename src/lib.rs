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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_vendor_full_mac_uppercase() {
        assert_eq!(
            lookup_mac_vendor("00:00:0C:AA:BB:CC"),
            Some("Cisco Systems, Inc")
        );
    }

    #[test]
    fn known_vendor_full_mac_lowercase() {
        assert_eq!(
            lookup_mac_vendor("00:00:0c:aa:bb:cc"),
            Some("Cisco Systems, Inc")
        );
    }

    #[test]
    fn known_vendor_full_mac_mixed_case() {
        assert_eq!(
            lookup_mac_vendor("00:00:0c:AA:BB:CC"),
            Some("Cisco Systems, Inc")
        );
    }

    #[test]
    fn known_vendor_oui_only() {
        assert_eq!(
            lookup_mac_vendor("00:00:0D:11:22:33"),
            Some("FIBRONICS LTD.")
        );
    }

    #[test]
    fn known_vendor_second_entry() {
        assert_eq!(
            lookup_mac_vendor("00:00:0D:11:22:33"),
            Some("FIBRONICS LTD.")
        );
    }

    #[test]
    fn unknown_vendor_returns_none() {
        assert_eq!(lookup_mac_vendor("FF:FF:FF:AA:BB:CC"), None);
    }

    #[test]
    fn unknown_oui_only_returns_none() {
        assert_eq!(lookup_mac_vendor("FF:FF:FF"), None);
    }
}

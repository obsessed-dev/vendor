use std::{env, error::Error, fs, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    let json = fs::read_to_string("data/mac-vendors-export.json")?;

    let entries: Vec<serde_json::Value> = serde_json::from_str(&json)?;

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest = Path::new(&out_dir).join("vendors.rs");

    let mut map = String::from("static MAC_VENDORS: phf::Map<&str, &str> = phf::phf_map! {\n");

    for entry in &entries {
        let (Some(prefix), Some(name)) =
            (entry["macPrefix"].as_str(), entry["vendorName"].as_str())
        else {
            continue;
        };
        let name = name.replace("\\", "\\\\").replace('"', "\\\"");
        map.push_str(&format!("   \"{prefix}\" => \"{name}\",\n"));
    }

    map.push_str("};\n");
    fs::write(&dest, &map)?;

    println!("cargo:rerun-if-changed=data/mac-vendors-export.json");

    Ok(())
}

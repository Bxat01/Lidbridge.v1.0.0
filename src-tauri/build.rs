fn main() {
    println!("cargo:rerun-if-changed=../.env");
    if let Ok(content) = std::fs::read_to_string("../.env") {
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let mut val = value.trim();
                if val.starts_with('"') && val.ends_with('"') {
                    val = &val[1..val.len() - 1];
                } else if val.starts_with('\'') && val.ends_with('\'') {
                    val = &val[1..val.len() - 1];
                }
                println!("cargo:rustc-env={}={}", key, val);
            }
        }
    }
    tauri_build::build()
}

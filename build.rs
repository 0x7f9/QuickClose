use std::path::Path;
use winres::WindowsResource;

fn main() {
    if std::env::var("PROFILE").unwrap_or_default() == "release" {
        println!("cargo:rustc-cfg=release_build");
    }
    
    let icon = "assets/logo.ico";
    let icon_path = Path::new(icon);
    println!("cargo:rerun-if-changed={}", icon_path.display());

    if !icon_path.exists() {
        eprintln!("App icon not found at {}", icon_path.display());
    } else {
        let mut res = WindowsResource::new();
        res.set_icon(icon);

        if let Err(e) = res.compile() {
            eprintln!("Failed to compile Windows resources: {}", e);
            std::process::exit(1);
        }
    }
}


use cc;
use std::fs;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let path = PathBuf::from("./node/out/Release");
    let libpath = fs::canonicalize(path)?;
    
    println!(
        "cargo:rustc-link-search={}",
        libpath.to_str().expect("Bad path")
    );
    
    println!("cargo:rustc-link-lib=node");

    Ok(())
}

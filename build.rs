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

    /*for lib in std::fs::read_dir("vendor/node/out/Release/lib")? {
        let entry = lib?;
        if entry.file_type()?.is_file() {
            let path = entry.path();

            if path.extension().unwrap() == "lib" {
                let filename = path.file_stem().expect("File has no extension.");

                println!(
                    "cargo:rustc-link-lib={}",
                    filename.to_str().expect("Invalid utf-8 filename.")
                );
            }

            
        }
    }*/

    // cc::Build::new().cpp(true).file("moon.cpp").compile("moon");

    Ok(())
}

use cc;

fn main() -> std::io::Result<()> {
    /* cc::Build::new()
    .cpp(true)
    .file("node.cpp")
    .compile("libmoon.a"); */

    println!("cargo:rustc-link-search=native=node/out/Release/lib");

    for lib in std::fs::read_dir("node/out/Release/lib")? {
        let entry = lib?;
        let path = entry.path();
        let filename = path.file_stem().expect("File has no extension.");

        println!(
            "cargo:rustc-link-lib=static={}",
            filename.to_str().expect("Invalid utf-8 filename.")
        );
        // println!("cargo:rustc-link-lib=static=libuv");
    }
    println!("cargo:rustc-link-lib=static=ws2_32");
    println!("cargo:rustc-link-lib=static=crypt32");
    
    println!("cargo:rustc-link-lib=static=dbghelp");
    println!("cargo:rustc-link-lib=static=user32");
    println!("cargo:rustc-link-lib=static=iphlpapi");
    println!("cargo:rustc-link-lib=static=winmm");
    println!("cargo:rustc-link-lib=static=psapi");

    Ok(())
}

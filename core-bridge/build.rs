fn main() {
    println!("cargo:rustc-link-search=native=snes9x");
    cxx_build::bridge("src/lib.rs")
        .file("snes9x/snes9x.cpp")
        .file("src/include/messages.cpp")
        .include("snes9x") // Add include directory for snes9x headers
        .std("c++14")
        .compile("snes9-wasm");

    println!("cargo:rerun-if-changed=snes9x/snes9x.h");
    println!("cargo:rerun-if-changed=snes9x/display.h");
    println!("cargo:rerun-if-changed=snes9x/messages.h");
    println!("cargo:rerun-if-changed=src/.");
}

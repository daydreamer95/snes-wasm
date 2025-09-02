//#![allow(non_upper_case_globals)]
//#![allow(non_camel_case_types)]
//#![allow(non_snake_case)]
//include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("snes-wasm/snes9x/display.h");
        include!("snes-wasm/snes9x/snes9x.h");
        include!("snes-wasm/snes9x/messages.h");

        fn S9xUsage();
    }

    extern "Rust" {
        fn rust_s9x_message(info: i32, usage: i32, description: &str);
    }
}

fn rust_s9x_message(info: i32, usage: i32, description: &str) {
    println!(
        "S9xMessage called: info={} usage={} description={}",
        info, usage, description
    );
}

fn main() {
    println!("Hello world from main!");

    let client = ffi::S9xUsage();
}

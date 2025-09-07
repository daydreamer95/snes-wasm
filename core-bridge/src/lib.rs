//#![allow(non_upper_case_globals)]
//#![allow(non_camel_case_types)]
//#![allow(non_snake_case)]
//include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("snes-wasm/snes9x/display.h");
        include!("snes-wasm/snes9x/snes9x.h");
        include!("snes-wasm/snes9x/messages.h");

        fn S9xUsage();
        //unsafe fn S9xMessage(info: i32, usage: i32, description: *const c_char);
    }

    extern "Rust" {
        fn rust_s9x_message(info: i32, usage: i32, description: &str);
    }
}

fn rust_s9x_message(info: i32, usage: i32, description: &str) {
    println!(
        "S9xMessage called from rust: info={} usage={} description={}",
        info, usage, description
    );
}

fn main() {
    let lib = "cargo:rustc-link-search=native=/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/include/c++";
    println!("cargo:rustc-link-search=native={}", lib);
    println!(
        "cargo:rustc-link-search=/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/include/c++/v1"
    );
}

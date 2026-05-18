fn main() {
    cxx_build::bridge("src/lib.rs")
        .std("c++17")
        .compile("chip8");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=post_build.rs");
}

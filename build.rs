fn main() {
    println!("cargo:rerun-if-changed=src/p2p_validator.zig");
    println!("cargo:rerun-if-changed=build.zig");
}
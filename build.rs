fn main() {
    #![cfg(target_arch = "x86_64")] {
        println!("cargo:rustc-link-arg=-Tlinker/x86_64.ld");
        println!("cargo:rerun-if-changed=linker/x86_64.ld");
    }
}
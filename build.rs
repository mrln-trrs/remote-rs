fn main() {
    println!("cargo:rerun-if-changed=src/native/windows_shim.cpp");
    println!("cargo:rerun-if-changed=src/native/windows_shim.h");
    
    cc::Build::new()
        .cpp(true)
        .file("src/native/windows_shim.cpp")
        .compile("windows_shim");
    
    println!("cargo:rustc-link-lib=user32");
    println!("cargo:rustc-link-lib=ole32");
    println!("cargo:rustc-link-lib=PowrProf");
}

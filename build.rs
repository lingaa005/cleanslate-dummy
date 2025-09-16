fn main() {
    println!("cargo:rustc-link-search=native=c_lib");
    println!("cargo:rustc-link-lib=static=lingaalib");
    println!("cargo:rustc-link-lib=advapi32");
    println!("cargo:rustc-link-lib=shell32");
}

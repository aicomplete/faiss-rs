fn main() {
    println!("cargo:rustc-link-search=./vendored/faiss/build/c_api");
    println!("cargo:rustc-link-lib=faiss_c");
}

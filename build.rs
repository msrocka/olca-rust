fn main() {
    // say the rust compiler that it should search in the
    // `bin` folder for linked libraries
    println!("cargo:rustc-link-search=./bin");
}

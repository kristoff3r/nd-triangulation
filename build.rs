#[cfg(not(feature = "docs-rs"))]
fn main() {
    cpp_build::Config::new()
        .compiler("/usr/bin/g++")
        .flag("-frounding-math")
        .flag("-std=c++14")
        .include("/usr/include/eigen3")
        .build("src/lib.rs");

    println!("cargo:rustc-link-search=/usr/lib");
    println!("cargo:rustc-link-lib=gmp");
    println!("cargo:rustc-link-lib=mpfr");
}

#[cfg(feature = "docs-rs")]
fn main() {}

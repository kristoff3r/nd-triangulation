#[cfg(not(feature = "docs-rs"))]
fn main() {
    let eigen = pkg_config::Config::new().atleast_version("3.3.0").probe("eigen3").unwrap();
    let mut conf = cpp_build::Config::new();
    conf.flag("-frounding-math");
    for path in eigen.include_paths {
        conf.include(path);
    }
    conf.build("src/lib.rs");

    println!("cargo:rustc-link-search=/usr/lib");
    println!("cargo:rustc-link-lib=CGAL");
    println!("cargo:rustc-link-lib=gmp");
    println!("cargo:rustc-link-lib=mpfr");
}

#[cfg(feature = "docs-rs")]
fn main() {}

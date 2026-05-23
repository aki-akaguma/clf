fn main() {
    let target = std::env::var("TARGET").unwrap_or_default();
    let use_void = target.contains("msvc") || target.contains("armv7-unknown-linux-musleabihf");

    let mut build = cc::Build::new();

    if use_void {
        build.file("src/c/clf-void.c");
    } else {
        build.file("src/c/clf.c");
    }

    build
        .flag_if_supported("-Wno-unused-function")
        .flag_if_supported("-Wno-unused-parameter")
        .compile("clf");

    println!("cargo:rerun-if-changed=src/c/clf.c");
    println!("cargo:rerun-if-changed=src/c/clf-void.c");
}

// build.rs

fn main() {
    compile_c_src();
}

fn compile_c_src() {
    println!("cargo:rerun-if-changed=src/c/clf.c");
    //
    let env_target = std::env::var("TARGET").unwrap();
    if env_target.as_str() != "armv7-unknown-linux-musleabihf" {
        cc::Build::new()
            .file("src/c/clf.c")
            .flag_if_supported("-Wno-unused-function")
            //.shared_flag(true)
            .static_flag(true)
            .compile("libclf.a");
    } else {
        std::env::set_var(
            "CC_armv7-unknown-linux-musleabihf",
            "arm-linux-gnueabihf-gcc",
        );
        cc::Build::new()
            .file("src/c/clf-void.c")
            .flag_if_supported("-Wno-unused-function")
            .flag_if_supported("-Wno-unused-parameter")
            //.shared_flag(true)
            .static_flag(true)
            .compile("libclf.a");
    }
}

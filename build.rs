// build.rs

fn main() {
    compile_c_src();
}

fn compile_c_src() {
    println!("cargo:rerun-if-changed=src/c/clf.c");
    //
    cc::Build::new()
        .file("src/c/clf.c")
        .flag_if_supported("-Wno-unused-function")
        //.shared_flag(true)
        .static_flag(true)
        .compile("libclf.a");
}

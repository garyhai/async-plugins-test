use libloading::Library;
use plugin_common::*;

#[cfg(feature = "rt-async-std")]
#[async_std::main]
async fn main() {
    main_test().await;
}

#[cfg(feature = "rt-tokio")]
#[tokio::main]
async fn main() {
    main_test().await;
}

async fn main_test() {
    let libname = "./target/debug/libasync_plugin.dylib";
    let library = Library::new(libname).unwrap();
    let decl = unsafe {
        library
            .get::<*mut LibDeclaration>(b"lib_declaration\0")
            .unwrap()
            .read()
    };
    let a = (decl.register)().await;

    println!("Hello, world! {}", a);
}

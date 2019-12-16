use libloading::Library;
use plugin_common::*;

#[async_std::main]
async fn main() {
    let libname = "/Users/gary/tmp/rust/debug/libasync_plugin.dylib";
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

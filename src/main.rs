use libloading::Library;
use plugin_common::*;
use std::cell::RefCell;

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

thread_local!(static BAR: RefCell<usize> = RefCell::new(4));

fn tls_get() -> usize {
    BAR.with(|f| {
        assert_eq!(*f.borrow(), 4);
        *f.borrow_mut() = 6;
    });
    BAR.with(|f| *f.borrow())
}

async fn main_test() {
    use tokio::runtime::Handle;
    let libname = "./target/debug/libasync_plugin.dylib";
    let library = Library::new(libname).unwrap();
    let decl = unsafe {
        library
            .get::<*mut LibDeclaration>(b"lib_declaration\0")
            .unwrap()
            .read()
    };
    let handle = Handle::current();
    dbg!(&handle);

    let a = (decl.register)(handle, tls_get).await;

    println!("Hello, world! {}", a);
}

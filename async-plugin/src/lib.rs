use plugin_common::*;
use std::{future::Future, pin::Pin};
use std::cell::RefCell;
use tokio::runtime::Handle;

thread_local!(static FOO: RefCell<usize> = RefCell::new(1));
thread_local! {
    static CONTEXT: RefCell<Option<Handle>> = RefCell::new(None)
}

pub(crate) fn current() -> Option<Handle> {
    CONTEXT.with(|ctx| ctx.borrow().clone())
}

#[no_mangle]
pub fn new_service(handle: Handle, tls: TlsGet) -> Pin<Box<dyn Future<Output = usize> + Send + Sync>> {
    CONTEXT.with(|x| *x.borrow_mut() = Some(handle.clone()));
    dbg!(tls());
    Box::pin(test_a())
}

#[no_mangle]
pub static lib_declaration: LibDeclaration = LibDeclaration {
    rustc_version: "abc",
    ns_version: "123",
    register: new_service,
};

#[cfg(feature = "rt-async-std")]
async fn test_a() -> usize {
    async_std::task::spawn(test_b()).await
}

#[cfg(feature = "rt-tokio")]
async fn test_a() -> usize {
    let handle = current().unwrap();
    dbg!(&handle);
    let n = handle.spawn(test_b()).await.unwrap();
    FOO.with(|f| *f.borrow()) + n
}

async fn test_b() -> usize {
    10
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

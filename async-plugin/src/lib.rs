use plugin_common::*;
use std::{future::Future, pin::Pin};

#[no_mangle]
pub fn new_service() -> Pin<Box<dyn Future<Output = usize> + Send + Sync>> {
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
    tokio::task::spawn(test_b()).await.unwrap()
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

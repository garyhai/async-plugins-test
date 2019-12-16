use std::{
    future::Future,
    pin::Pin,
};
use plugin_common::*;
use async_std::task::spawn;

#[no_mangle]
pub extern "Rust" fn new_service() -> Pin<Box<dyn Future<Output = usize> + Send + Sync>> {
    Box::pin(test_a())
}

#[no_mangle]
pub static lib_declaration: LibDeclaration =
    LibDeclaration {
        rustc_version: "abc",
        ns_version: "123",
        register: new_service,
    };

async fn test_a() -> usize {
    spawn(test_b());
    10
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

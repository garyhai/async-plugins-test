use std::{
    future::Future,
    pin::Pin,
};
use tokio::runtime::Handle;

pub type TlsGet = extern "Rust" fn() -> usize;

pub type FnRegMethod = extern "Rust" fn(Handle, TlsGet) -> Pin<Box<dyn Future<Output = usize> + Send + Sync>>;


#[derive(Copy, Clone)]
pub struct LibDeclaration {
    pub ns_version: &'static str,
    pub rustc_version: &'static str,
    pub register: FnRegMethod,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

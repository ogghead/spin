#![allow(missing_docs)]

wasmtime::component::bindgen!({
    path: "../../wit/preview2",
    world: "reactor",
    async: true
});

pub use fermyon::spin::*;

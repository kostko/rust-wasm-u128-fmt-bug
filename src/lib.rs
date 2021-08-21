#![feature(wasm_abi)]

#[no_mangle]
pub extern "wasm" fn test() {
    format!("boom: {}", 0u128);
}


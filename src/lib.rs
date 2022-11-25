mod utils;

use wasm_bindgen::prelude::*;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    utils::set_panic_hook();
    Ok(())
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "crypto", "subtle"])]
    async fn digest(algorithm: &str, data: js_sys::Uint8Array) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "crypto"])]
    fn getRandomValues(data: &mut [u8]);

    // #[wasm_bindgen(js_namespace = ["window"])]
    // fn buffer2str(buffer: JsValue) -> JsValue;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


#[wasm_bindgen]
pub async fn sign(url: &str) -> JsValue {
    let salt = "asy8";
    unsafe {
        let mut random = [0u8; 8];
        getRandomValues(&mut random);
        let random_str = hex::encode(random.to_vec());
        let data_to_hash = random_str + url + salt;
        // log(&data_to_hash);
        let data_to_hash = js_sys::Uint8Array::view(data_to_hash.as_bytes());
        let result = digest("SHA-256", data_to_hash).await;
        let result = js_sys::ArrayBuffer::from(result);
        let result = js_sys::Uint8Array::new(&result);
        let result = result.to_vec();
        let result = hex::encode(random.to_vec()) + &hex::encode(result);
        // log(&result);
        JsValue::from_str(&result)
        // let buffer = digest("SHA-256", data_to_hash).await;
        // buffer2str(buffer)
    }
}

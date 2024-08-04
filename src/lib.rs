use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn error(s: &str);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn readFile(path: JsValue) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(catch)]
    async fn writeFile(path: JsValue, content: JsValue) -> Result<(), JsValue>;
}

#[wasm_bindgen]
pub async fn copy_file(from: JsValue, to: JsValue) -> Result<(), JsValue> {
    let content = readFile(from).await.unwrap();
    writeFile(to, content).await?;
    log("File copied successfully!");
    Ok(())
}

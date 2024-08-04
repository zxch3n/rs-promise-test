use futures::FutureExt;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

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
    log("Before reading file");
    let content = readFile(from).await.unwrap();
    log("Read file");
    writeFile(to, content).await?;
    log("File copied successfully!");
    Ok(())
}

#[wasm_bindgen]
pub fn copy_file_sync(from: JsValue, to: JsValue) -> Result<(), JsValue> {
    log("Spawn local");
    spawn_local(copy_file(from, to).map(|_| ()));
    log("Spawned local");
    Ok(())
}

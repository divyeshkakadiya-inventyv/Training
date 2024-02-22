use table_task::make_table::make;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

pub mod table_task;

#[wasm_bindgen]
extern "C" {
     fn readData() -> String;
}

#[wasm_bindgen]
pub fn get_table()-> String  {
    let data = readData();
//    let res = make(data.as_str());
//    data
     data
}

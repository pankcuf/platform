use crate::buffer::Buffer;
use dpp::identifier::Identifier;
use dpp::prelude::Revision;
use js_sys::Number;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name=InvalidIdentityRevisionError)]
pub struct InvalidIdentityRevisionErrorWasm {
    identity_id: Identifier,
    current_revision: Revision,
    code: u32,
}

#[wasm_bindgen(js_class=InvalidIdentityRevisionError)]
impl InvalidIdentityRevisionErrorWasm {
    #[wasm_bindgen(js_name=getIdentityId)]
    pub fn identity_id(&self) -> Buffer {
        Buffer::from_bytes(self.identity_id.as_bytes())
    }

    #[wasm_bindgen(js_name=getCurrentRevision)]
    pub fn current_revision(&self) -> Number {
        // It might be overflow
        Number::from(self.current_revision as f64)
    }

    #[wasm_bindgen(js_name=getCode)]
    pub fn get_code(&self) -> u32 {
        self.code
    }
}

impl InvalidIdentityRevisionErrorWasm {
    pub fn new(identity_id: Identifier, current_revision: Revision, code: u32) -> Self {
        Self {
            identity_id,
            current_revision,
            code,
        }
    }
}

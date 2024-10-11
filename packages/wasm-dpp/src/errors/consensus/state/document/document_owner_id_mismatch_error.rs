use crate::buffer::Buffer;
use dpp::errors::consensus::codes::ErrorWithCode;
use dpp::errors::consensus::state::document::document_owner_id_mismatch_error::DocumentOwnerIdMismatchError;
use dpp::errors::consensus::ConsensusError;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name=DocumentOwnerIdMismatchError)]
pub struct DocumentOwnerIdMismatchErrorWasm {
    inner: DocumentOwnerIdMismatchError,
}

impl From<&DocumentOwnerIdMismatchError> for DocumentOwnerIdMismatchErrorWasm {
    fn from(e: &DocumentOwnerIdMismatchError) -> Self {
        Self { inner: e.clone() }
    }
}

#[wasm_bindgen(js_class=DocumentOwnerIdMismatchError)]
impl DocumentOwnerIdMismatchErrorWasm {
    #[wasm_bindgen(js_name=getDocumentId)]
    pub fn document_id(&self) -> Buffer {
        Buffer::from_bytes(self.inner.document_id().as_bytes())
    }

    #[wasm_bindgen(js_name=getDocumentOwnerId)]
    pub fn document_owner_id(&self) -> Buffer {
        Buffer::from_bytes(self.inner.document_owner_id().as_bytes())
    }

    #[wasm_bindgen(js_name=getExistingDocumentOwnerId)]
    pub fn existing_document_owner_id(&self) -> Buffer {
        Buffer::from_bytes(self.inner.existing_document_owner_id().as_bytes())
    }

    #[wasm_bindgen(js_name=getCode)]
    pub fn get_code(&self) -> u32 {
        ConsensusError::from(self.inner.clone()).code()
    }

    #[wasm_bindgen(getter)]
    pub fn message(&self) -> String {
        self.inner.to_string()
    }
}

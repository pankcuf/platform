use crate::identity::core_script::CoreScript;
use crate::prelude::Revision;
use dashcore::{PubkeyHash, ScriptBuf};
use platform_value::string_encoding::{encode, Encoding};
use platform_value::BinaryData;
use platform_value::{platform_value, Identifier, Value};
use platform_version::version::LATEST_VERSION;
use serde_json::{json, Value as JsonValue};

use crate::withdrawal::Pooling;
use crate::state_transition::StateTransitionType;

pub fn identity_credit_withdrawal_transition_fixture_raw_object() -> Value {
    platform_value!({
        "protocolVersion": LATEST_VERSION,
        "type": StateTransitionType::IdentityCreditWithdrawal as u8,
        "identityId": Identifier::from([1_u8; 32]),
        "amount": 1042u64,
        "coreFeePerByte": 3u32,
        "pooling": Pooling::Never as u8,
        "outputScript": CoreScript::new(ScriptBuf::new_p2pkh(&PubkeyHash::from_hex("0000000000000000000000000000000000000000").unwrap())),
        "revision": 1 as Revision,
        "signaturePublicKeyId": 0u32,
        "signature": BinaryData::new(vec![0_u8; 65]),
    })
}

pub fn identity_credit_withdrawal_transition_fixture_json() -> JsonValue {
    json!({
        "protocolVersion": LATEST_VERSION,
        "type": StateTransitionType::IdentityCreditWithdrawal,
        "identityId": encode(&[1_u8; 32], Encoding::Base58),
        "amount": 1042,
        "coreFeePerByte": 3,
        "pooling": Pooling::Never,
        "outputScript": encode(&ScriptBuf::new_p2pkh(&PubkeyHash::from_hex("0000000000000000000000000000000000000000").unwrap()).to_bytes(), Encoding::Base64),
        "signature": encode(&[0_u8; 65], Encoding::Base64),
        "signaturePublicKeyId": 0,
        "revision": 1,
    })
}

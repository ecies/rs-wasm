use ecies::{
    decrypt as _decrypt, encrypt as _encrypt, utils::generate_keypair as _generate_keypair,
};
use js_sys::{Array, Uint8Array};
use wasm_bindgen::prelude::{wasm_bindgen, JsError};

/// Generate a `(SecretKey, PublicKey)` pair
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn generateKeypair() -> Array {
    let (sk, pk) = _generate_keypair();
    let (sk, pk) = (&sk.serialize()[..], &pk.serialize_compressed()[..]);
    let (sk, pk) = (Uint8Array::from(sk), Uint8Array::from(pk));

    let ret = Array::new();
    ret.push(&sk);
    ret.push(&pk);
    ret
}

/// Encrypt a message by a public key
#[wasm_bindgen]
pub fn encrypt(pk: &[u8], msg: &[u8]) -> Result<Uint8Array, JsError> {
    _encrypt(pk, msg)
        .map(|v| Uint8Array::from(v.as_slice()))
        .map_err(|e| JsError::new(&e.to_string()))
}

/// Decrypt a message by a secret key
#[wasm_bindgen]
pub fn decrypt(sk: &[u8], msg: &[u8]) -> Result<Uint8Array, JsError> {
    _decrypt(sk, msg)
        .map(|v| Uint8Array::from(v.as_slice()))
        .map_err(|e| JsError::new(&e.to_string()))
}

#[cfg(all(test, target_arch = "wasm32"))]
mod tests {
    use js_sys::Uint8Array;

    use wasm_bindgen::JsValue;
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::{_generate_keypair, decrypt, encrypt, generateKeypair};

    const MSG: &str = "hello-world🌎";

    fn __enc_dec(sk: &[u8], pk: &[u8], msg: &[u8]) {
        let encrypted = encrypt(pk, msg).ok().unwrap().to_vec();
        assert_eq!(
            msg,
            decrypt(sk, &encrypted).ok().unwrap().to_vec().as_slice()
        );
    }

    #[wasm_bindgen_test]
    fn test_rust() {
        let (sk, pk) = _generate_keypair();
        let (sk, pk) = (&sk.serialize(), &pk.serialize_compressed());
        __enc_dec(sk, pk, MSG.as_bytes());
    }

    #[wasm_bindgen_test]
    fn test_wasm() {
        let pair = generateKeypair();
        let sk = Uint8Array::from(pair.get(0)).to_vec();
        let pk = Uint8Array::from(pair.get(1)).to_vec();
        let msg = Uint8Array::from(MSG.as_bytes()).to_vec();

        __enc_dec(&sk, &pk, &msg);
    }

    #[wasm_bindgen_test]
    fn test_wasm_error() {
        let pk = Uint8Array::new(&JsValue::from(33)).to_vec();
        assert!(encrypt(&pk, MSG.as_bytes()).is_err());
    }
}

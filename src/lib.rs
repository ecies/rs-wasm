use ecies::{decrypt as _decrypt, encrypt as _encrypt};
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn encrypt(receiver_pub: &[u8], msg: &[u8]) -> Option<Uint8Array> {
    // TODO: handle error
    _encrypt(receiver_pub, msg)
        .map(|v| Uint8Array::from(v.as_slice()))
        .ok()
}

#[wasm_bindgen]
pub fn decrypt(receiver_sec: &[u8], msg: &[u8]) -> Option<Uint8Array> {
    // TODO: handle error
    _decrypt(receiver_sec, msg)
        .map(|v| Uint8Array::from(v.as_slice()))
        .ok()
}

#[cfg(all(test, target_arch = "wasm32"))]
mod tests {
    use ecies::utils::generate_keypair;
    use wasm_bindgen_test::*;

    use super::*;

    const MSG: &str = "helloworld";

    fn test_enc_dec(sk: &[u8], pk: &[u8]) {
        let msg = MSG.as_bytes();
        let encrypted = encrypt(pk, msg).unwrap().to_vec();
        assert_eq!(msg, decrypt(sk, &encrypted).unwrap().to_vec().as_slice());
    }

    #[wasm_bindgen_test]
    fn test_random_keypair() {
        let (sk, pk) = generate_keypair();
        let (sk, pk) = (&sk.serialize(), &pk.serialize());
        test_enc_dec(sk, pk);
    }
}

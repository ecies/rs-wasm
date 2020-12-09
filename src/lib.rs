use ecies::{
    decrypt as _decrypt, encrypt as _encrypt, utils::generate_keypair as _generate_keypair,
};
use js_sys::{Array, Uint8Array};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn generate_keypair() -> Array {
    let (sk, pk) = _generate_keypair();
    let (sk, pk) = (sk.serialize(), pk.serialize_compressed());

    let (sk, pk) = (Uint8Array::from(&sk[..]), Uint8Array::from(&pk[..]));

    let ret = Array::new();
    ret.push(&sk);
    ret.push(&pk);
    ret
}

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
    use wasm_bindgen_test::*;

    use super::*;

    const MSG: &str = "helloworld";

    fn test_enc_dec(sk: &[u8], pk: &[u8]) {
        let msg = MSG.as_bytes();
        let encrypted = encrypt(pk, msg).unwrap().to_vec();
        assert_eq!(msg, decrypt(sk, &encrypted).unwrap().to_vec().as_slice());
    }

    #[wasm_bindgen_test]
    fn test_rust() {
        let (sk, pk) = _generate_keypair();
        let (sk, pk) = (&sk.serialize(), &pk.serialize());
        test_enc_dec(sk, pk);
    }

    #[wasm_bindgen_test]
    fn test_wasm() {
        let data = "😀😀😀😀".as_bytes();
        let data_js = Uint8Array::from(&data[..]);

        let pair = generate_keypair();
        let (sk, pk) = (Uint8Array::from(pair.get(0)), Uint8Array::from(pair.get(1)));

        let encrypted = encrypt(&pk.to_vec(), data).unwrap();
        let decrypted = decrypt(&sk.to_vec(), &encrypted.to_vec()).unwrap();
        assert_eq!(data_js.to_vec(), decrypted.to_vec());
    }
}

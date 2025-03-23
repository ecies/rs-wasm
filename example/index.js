// need to build with `wasm-pack build --target web`
// check vite.config.ts as well
import init, * as ecies from "ecies-wasm";
import "./style.css";
import { bytesToHex } from "./utils";

let sk, pk;

init().then(() => {
  [sk, pk] = ecies.generateKeypair();
});


const encoder = new TextEncoder();
const decoder = new TextDecoder();
const text = "hello ecies-wasm🔒";

export function setup(encryptedElement, textElement, decryptedElement) {
  let encrypted;

  encryptedElement.innerHTML = "click me to encrypt";
  textElement.innerHTML = text;
  decryptedElement.innerHTML = "click me to decrypt";

  const _encrypt = () => {
    encrypted = ecies.encrypt(pk, encoder.encode(text));
    encryptedElement.innerHTML = "encrypted:";
    textElement.innerHTML = `<code>${bytesToHex(encrypted)}</code>`;
    decryptedElement.innerHTML = "click me to decrypt";
  };

  const _decrypt = () => {
    encryptedElement.innerHTML = "click me to encrypt";
    if (encrypted) {
      const decrypted = decoder.decode(ecies.decrypt(sk, encrypted));
      textElement.innerHTML = `${decrypted}`;
      decryptedElement.innerHTML = "decrypted:";
      encrypted = undefined;
    } else {
      textElement.innerHTML = "click encrypt button first";
    }
  };
  encryptedElement.addEventListener("click", () => _encrypt());
  decryptedElement.addEventListener("click", () => _decrypt());
}

document.querySelector("#app").innerHTML = `
  <div>
    <h1>Hello ecies-wasm!</h1>
    <div class="card">
      <button id="encrypted" type="button"></button>
      <button id="decrypted" type="button"></button>
    </div>
    <p id="text"></p>
  </div>
`;

setup(
  document.querySelector("#encrypted"),
  document.querySelector("#text"),
  document.querySelector("#decrypted")
);

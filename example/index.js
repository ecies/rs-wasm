// need to build with `wasm-pack build --target web`
// import init, * as ecies from "../pkg/ecies_wasm";
// check vite.config.js as well
import init, * as ecies from "ecies-wasm";

init();

const encoder = new TextEncoder();
const data = encoder.encode("hello ecies🔒");

function checkOk() {
  const [sk, pk] = ecies.generateKeypair();

  const encrypted = ecies.encrypt(pk, data);
  const decrypted = ecies.decrypt(sk, encrypted);

  const decoder = new TextDecoder();
  alert(`decrypted: ${decoder.decode(decrypted)}`);

  if (decrypted.toString("hex") === data.toString("hex")) {
    alert("call wasm encrypt decrypt ok");
  } else {
    alert("call wasm encrypt decrypt failed");
  }
}

function checkError() {
  const pk = Uint8Array.from([0]);
  try {
    ecies.encrypt(pk, data);
  } catch (e) {
    alert(e);
  }
}

document.querySelector("#app").innerHTML = `
  <h1>WASM Test</h1>
  <button id="ok">Check ok</button>
  <button id="error">Check error</button>
`;

document.getElementById("ok").addEventListener("click", () => {
  checkOk();
});
document.getElementById("error").addEventListener("click", () => {
  checkError();
});

window.addEventListener("error", (event) => {
  // catch all other errors
  console.error(event);
});

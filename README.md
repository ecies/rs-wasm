# ecies-wasm

[![License](https://img.shields.io/github/license/ecies/rs-wasm.svg)](https://github.com/ecies/rs-wasm)
[![NPM Package](https://img.shields.io/npm/v/ecies-wasm.svg)](https://www.npmjs.com/package/ecies-wasm)
[![CI](https://github.com/ecies/rs-wasm/workflows/Build%20wasm/badge.svg)](https://github.com/ecies/rs-wasm/actions)
[![CD](https://github.com/ecies/rs-wasm/workflows/Publish%20wasm/badge.svg)](https://github.com/ecies/rs-wasm/actions)

A WASM bind for [eciesrs](https://github.com/ecies/rs).

## Install

```bash
npm install ecies-wasm
```

## Usage

```js
import * as wasm from "ecies-wasm";

const fromHexString = (hexString) =>
  new Uint8Array(hexString.match(/.{1,2}/g).map((byte) => parseInt(byte, 16)));

const hex_pk =
  "98afe4f150642cd05cc9d2fa36458ce0a58567daeaf5fde7333ba9b403011140a4e28911fcf83ab1f457a30b4959efc4b9306f514a4c3711a16a80e3b47eb58b";
const hex_sk = "95d3c5e483e9b1d4f5fc8e79b2deaf51362980de62dbb082a9a4257eef653d7d";

const pk = Uint8Array.from(fromHexString(hex_pk, "hex"));
const sk = Uint8Array.from(fromHexString(hex_sk, "hex"));

const data = Uint8Array.from([1, 2, 3, 4]);
const encrypted = wasm.encrypt(pk, data);
const data_decrypted = wasm.decrypt(sk, encrypted);
console.log(data_decrypted);
```

Check [this example](https://github.com/ecies/wasm-example) for more details.

## API

```js
function encrypt(receiver_pub: Uint8Array, msg: Uint8Array): Uint8Array | undefined;
function decrypt(receiver_sec: Uint8Array, msg: Uint8Array): Uint8Array | undefined;
```

## Build and test

```bash
wasm-pack build
wasm-pack test --node
```

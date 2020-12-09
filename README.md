# ecies-wasm

[![License](https://img.shields.io/github/license/ecies/rs-wasm.svg)](https://github.com/ecies/rs-wasm)
[![NPM Package](https://img.shields.io/npm/v/ecies-wasm.svg)](https://www.npmjs.com/package/ecies-wasm)
[![CI](https://github.com/ecies/rs-wasm/workflows/Build%20wasm/badge.svg)](https://github.com/ecies/rs-wasm/actions)
[![CD](https://github.com/ecies/rs-wasm/workflows/Publish%20wasm/badge.svg)](https://github.com/ecies/rs-wasm/actions)

A WASM binding for [eciesrs](https://github.com/ecies/rs).

## Install

```bash
npm install ecies-wasm
```

## Usage

```js
import * as ecies from "ecies-wasm";

const [sk, pk] = ecies.generate_keypair();
const data = Uint8Array.from([1, 2, 3, 4]);

const encrypted = ecies.encrypt(pk, data);
const decrypted = ecies.decrypt(sk, encrypted);
alert("decrypted: " + decrypted);
```

Check [this example](https://github.com/ecies/wasm-example) for more details.

## API

```ts
function encrypt(receiver_pub: Uint8Array, msg: Uint8Array): Uint8Array | undefined;
function decrypt(receiver_sec: Uint8Array, msg: Uint8Array): Uint8Array | undefined;
```

## Build and test

```bash
wasm-pack build
wasm-pack test --node
```

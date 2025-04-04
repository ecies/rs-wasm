# ecies-wasm

[![License](https://img.shields.io/github/license/ecies/rs-wasm.svg)](https://github.com/ecies/rs-wasm)
[![NPM Package](https://img.shields.io/npm/v/ecies-wasm.svg)](https://www.npmjs.com/package/ecies-wasm)
[![NPM Downloads](https://img.shields.io/npm/dm/ecies-wasm)](https://npm-stat.link/ecies-wasm)
[![Install size](https://packagephobia.com/badge?p=ecies-wasm)](https://packagephobia.com/result?p=ecies-wasm)
[![CI](https://img.shields.io/github/actions/workflow/status/ecies/rs-wasm/ci.yml)](https://github.com/ecies/rs-wasm/actions)

A WASM binding for [eciesrs](https://github.com/ecies/rs).

## Install

```bash
npm install ecies-wasm
```

## Usage

```js
import init, * as ecies from "ecies-wasm";

const encoder = new TextEncoder();
const decoder = new TextDecoder();

// if built with vite without plugin
// can also run with bun/deno
init().then(() => {
    const [sk, pk] = ecies.generateKeypair();
    const data = encoder.encode("hello ecies🔒");
    const encrypted = ecies.encrypt(pk, data);
    const decrypted = decoder.decode(ecies.decrypt(sk, encrypted));
    console.log("decrypted: " + decrypted);
});
```

Check [the example](./example) for how to use it in browsers.

## API

```ts
export function generateKeypair(): Uint8Array[]; // [sk, pk]
export function encrypt(pk: Uint8Array, msg: Uint8Array): Uint8Array;
export function decrypt(sk: Uint8Array, msg: Uint8Array): Uint8Array;
```

## Build and test

```bash
wasm-pack build
wasm-pack build --target web
wasm-pack test --node
```

## Changelog

See [CHANGELOG.md](./CHANGELOG.md).

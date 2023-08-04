# ecies-wasm

[![License](https://img.shields.io/github/license/ecies/rs-wasm.svg)](https://github.com/ecies/rs-wasm)
[![NPM Package](https://img.shields.io/npm/v/ecies-wasm.svg)](https://www.npmjs.com/package/ecies-wasm)
[![CI](https://img.shields.io/github/actions/workflow/status/ecies/rs-wasm/ci.yml)](https://github.com/ecies/rs-wasm/actions)

A WASM binding for [eciesrs](https://github.com/ecies/rs).

## Install

```bash
npm install ecies-wasm
```

## Usage

```js
import init, * as ecies from "ecies-wasm";

init(); // if built with vite without plugin

const encoder = new TextEncoder();
const data = encoder.encode("hello ecies🔒");

const [sk, pk] = ecies.generateKeypair();
const encrypted = ecies.encrypt(pk, data);
const decrypted = ecies.decrypt(sk, encrypted);
alert("decrypted: " + decrypted);
```

Check [the example](./example) for how to use it in browsers.

## API

```ts
export function generateKeypair(): Array<Uint8Array>; // [sk, pk]
export function encrypt(pk: Uint8Array, msg: Uint8Array): Uint8Array;
export function decrypt(sk: Uint8Array, msg: Uint8Array): Uint8Array;
```

## Build and test

```bash
wasm-pack build
wasm-pack test --node
```

## Changelog

See [CHANGELOG.md](./CHANGELOG.md).

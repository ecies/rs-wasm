# ecies wasm example

This example shows how to use `ecies-wasm` with [vite](https://vitejs.dev/).

## Install

`yarn install`

## Run dev server

`yarn dev`

## Build and preview

`yarn build && yarn preview`

## Build local wasm package (optional)

Make sure you have [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) installed. Then `wasm-pack build --target web`.

After `pkg` folder is generated, update `import` in `index.js` as:

```ts
import init, * as ecies from "../pkg/ecies_wasm";
```

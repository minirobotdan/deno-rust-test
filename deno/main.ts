const wasmCode = await Deno.readFile('../target/wasm32-unknown-unknown/debug/deno_rust_test.wasm');
const wasmModule = new WebAssembly.Module(wasmCode);
const wasmInstance = new WebAssembly.Instance(wasmModule);

const { square } = wasmInstance.exports;

console.log(square(3));
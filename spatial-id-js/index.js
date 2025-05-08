// spatial-id-js: spatial-id-wasmのラッパー
import init, { WasmLatLon } from 'spatial-id-wasm';

export async function initSpatialIdWasm() {
  await init();
}

export { WasmLatLon };

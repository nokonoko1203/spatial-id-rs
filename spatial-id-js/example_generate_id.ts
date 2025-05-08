import { generate_spatial_id } from '../spatial-id-wasm/pkg/spatial_id_wasm.js';

const output = document.getElementById('output');
try {
  // サンプル座標: lat, lon, alt, zoom
  const lat = 35.0;
  const lon = 135.0;
  const alt = 0.0;
  const zoom = 25;
  const spatialId = generate_spatial_id(lat, lon, alt, zoom);
  if (output) {
    output.textContent = `Spatial ID: ${spatialId}`;
  }
  console.log('Spatial ID:', spatialId);
} catch (e) {
  if (output) output.textContent = `エラー: ${String(e)}`;
  console.error(e);
}

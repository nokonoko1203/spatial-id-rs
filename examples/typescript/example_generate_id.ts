import { generate_spatial_id } from 'spatial-id-js';

const output = document.getElementById('output');
try {
  const lat = 0.0;
  const lon = 0.0;
  const alt = 10.0;
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

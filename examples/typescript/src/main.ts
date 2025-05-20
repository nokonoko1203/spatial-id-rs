import { generate_spatial_id } from "spatial-id-wasm";

async function main() {
  const form = document.getElementById("spatial-form") as HTMLFormElement;
  const resultDiv = document.getElementById("result")!;
  const errorDiv = document.getElementById("error")!;

  form.onsubmit = (e) => {
    e.preventDefault();
    errorDiv.textContent = "";
    resultDiv.textContent = "";

    const formData = new FormData(form);
    const lat = Number(formData.get("lat"));
    const lon = Number(formData.get("lon"));
    const alt = Number(formData.get("alt"));
    const zoom = Number(formData.get("zoom"));

    if (
      isNaN(lat) || isNaN(lon) || isNaN(alt) || isNaN(zoom) ||
      zoom < 0 || zoom > 26
    ) {
      errorDiv.textContent = "入力値が不正です。";
      return;
    }

    try {
      const spatialId = generate_spatial_id(lat, lon, alt, zoom);
      resultDiv.textContent = `Spatial ID: ${spatialId}`;
    } catch (err) {
      errorDiv.textContent = `エラー: ${(err as Error).message}`;
    }
  };
}

main().catch((err) => {
  document.querySelector<HTMLDivElement>("#app")!.innerHTML =
    `<div style="color:red;">初期化エラー: ${(err as Error).message}</div>`;
});

# spatial-id-rs プロジェクト

Rust × WebAssembly × TypeScript 空間ID生成ライブラリ

## プロジェクト構成

```
spatial-id-rs/           # Rustコアロジック（純粋な型・処理のみ）
spatial-id-wasm/         # Wasmバインディング（wasm-bindgen/wasm-pack用）
spatial-id-js/           # npm/TypeScriptラッパー・型定義・利用サンプル
```

- Rustコアは`spatial-id-rs`、Wasmバインディングは`spatial-id-wasm`、npm/TSラッパーは`spatial-id-js`に分離

## ビルド・利用方法

### 1. Rustコアロジックのビルド

```
cargo build -p spatial-id-rs
```

### 2. WebAssemblyバインディングのビルド

#### Node.js用 (CJS)
```
cd spatial-id-wasm
wasm-pack build --target nodejs
```

#### バンドラ/ブラウザ用 (ESM)
```
cd spatial-id-wasm
wasm-pack build --target bundler
```

成果物は`spatial-id-wasm/pkg/`に出力されます。

### 3. フロントエンド（Vite+TypeScript）での利用

- `spatial-id-js` ディレクトリに、npmパッケージ雛形・型定義・利用サンプルを配置
- `example_generate_id.ts` … Vite/TypeScript/ブラウザ用サンプル
- `index.html` … サンプルのエントリHTML

#### 例: Vite + TypeScript + wasm-pack(bundler)

1. wasm-packでESMターゲットをビルド
   ```sh
   cd spatial-id-wasm
   wasm-pack build --target bundler
   ```
2. `spatial-id-js` ディレクトリで依存をインストール
   ```sh
   pnpm install
   ```
3. Vite開発サーバーを起動
   ```sh
   pnpm exec vite
   ```
4. ブラウザで http://localhost:5173/ にアクセス

#### example_generate_id.ts
```ts
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
```

#### index.html
```html
<!DOCTYPE html>
<html lang="ja">
<head>
  <meta charset="UTF-8">
  <title>spatial-id-wasm Viteサンプル</title>
</head>
<body>
  <h1>spatial-id-wasm Vite/TypeScript サンプル</h1>
  <pre id="output"></pre>
  <script type="module" src="./example_generate_id.ts"></script>
</body>
</html>
```

- Viteは`vite-plugin-wasm`でwasmファイルのimportを自動で解決します。
- サンプルは`spatial-id-js`ディレクトリ直下で起動・動作します。
- SSRやNode.jsではなく、**フロントエンド/ブラウザでの利用を想定**しています。


## 開発の流れ

1. Rustコアロジック（spatial-id-rs）を実装・テスト
2. Wasmバインディング（spatial-id-wasm）でエクスポート関数・型を設計
3. wasm-packでターゲットごとにビルド
4. spatial-id-jsでnpmラッパー・型定義・利用サンプルを整備
5. 必要に応じて型定義や成果物をnpm公開

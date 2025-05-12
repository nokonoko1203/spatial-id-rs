# spatial-id-rs

Rust用の空間ID生成ライブラリ

## プロジェクト構成

```
spatial-id-rs/           # Rustコアロジック
spatial-id-wasm/         # Wasmバインディング
spatial-id-js/           # npm/TypeScriptバインディング
spatial-id-py/           # Pythonバインディング
```

## ビルド・利用方法

### Rustコアロジックのビルド

```
cargo build -p spatial-id-rs
```

### WebAssemblyバインディングのビルド

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

### フロントエンド（Vite+TypeScript）での利用

- `examples/typescript`ディレクトリに、npmパッケージ雛形・型定義・利用サンプルを配置
- `example_generate_id.ts`: Vite/TypeScript/ブラウザ用サンプル
- `index.html`: サンプルのエントリHTML

#### 開発の流れ

1. wasm-packでESMターゲットをビルド
   ```sh
   cd spatial-id-wasm
   wasm-pack build --target bundler
   ```
2. `spatial-id-js`ディレクトリで依存関係をインストール
   ```sh
   cd spatial-id-js
   pnpm install
   ```
3. 開発: 利用したい`spatial-id-wasm`のAPIなどがあれば定義してexportする
4. ビルド
   ```sh
   cd spatial-id-js
   pnpm run build
   ```
5. `examples/typescript`ディレクトリで依存関係をインストール
   ```sh
   cd examples/typescript
   pnpm install
   ```
6. Vite開発サーバーを起動
   ```sh
   pnpm exec vite
   ```
7. ブラウザで http://localhost:5173/ にアクセス

### Pythonバインディング（spatial-id-py）について

#### 概要

- `examples/python`ディレクトリに、Pythonパッケージ雛形・型定義・利用サンプルを配置
- `example_generate_id.py`: Python用サンプル
- Rustコア（spatial-id-rs）の空間ID生成ロジックをPyO3でPythonバインディング化

#### 開発の流れ

1. `spatial-id-py`ディレクトリで依存関係をインストール
   ```sh
   cd spatial-id-py
   uv sync
   ```
2. 開発: 利用したい`spatial-id-rs`のAPIなどがあれば`spatial-id-py/src/lib.rs`に定義する
3. maturinでPythonパッケージをビルド
   ```sh
   uv run maturin develop
   ```
4. `examples/python`ディレクトリで依存関係をインストール
   ```sh
   cd examples/python
   uv sync
   ```
5. Pythonスクリプトを実行
   ```sh
   uv run python example_generate_id.py
   ```

#### Rust/wasm/TypeScript連携のそれぞれの特徴

- JS/TypeScript連携はwasm-bindgen/wasm-pack経由でWasmバイナリを生成してパッケージで利用する
- Python連携はPyO3+maturinで直接Python拡張モジュールを生成
- どちらもRustコアロジック（spatial-id-rs）を再利用し、API設計も統一


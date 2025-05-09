# spatial-id-py

Rust製空間ID生成ロジック（spatial-id-rs）をPythonから利用するためのPyO3バインディングです。

## 特長
- Rustの高速・型安全な空間ID生成をPythonから手軽に呼び出し可能
- maturinを用いたビルド・pip/pypi配布対応
- 型ヒント・docstringつき

## セットアップ・開発フロー（uv + pyproject.toml依存管理・activate不要）

### 1. Rust依存のインストール
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Python依存・仮想環境の自動セットアップ（uv + pyproject.toml）
```sh
uv pip install -r pyproject.toml
```
- 依存・仮想環境はpyproject.tomlの[project.dependencies]に従い自動解決されます。
- activateやpip installは不要です。

### 3. Rustバインディングのビルド
```sh
uv run maturin develop
```

## 使い方サンプル
```python
from spatial_id_py import generate_spatial_id

id = generate_spatial_id(35.0, 135.0, 0.0, 25)
print(id)
```

## 開発・テスト
```sh
uv run python example_generate_id.py
uv run pytest tests/
```

---
- Python 3.12系での動作を推奨します。
- 依存管理・実行はすべてuv純正コマンド（activate不要・uv run推奨）で統一しています。
- 依存追加は `uv add パッケージ名` でpyproject.tomlへ自動反映されます。

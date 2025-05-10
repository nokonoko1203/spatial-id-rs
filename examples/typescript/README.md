# spatial-id-js TypeScript 外部利用サンプル

このディレクトリは `spatial-id-js` をnpmパッケージとして外部から利用するための独立サンプルです。

## セットアップ

1. 依存インストール

```sh
pnpm install
```

2. spatial-id-jsをリポジトリルートからローカルリンク

```sh
cd ../../spatial-id-js
pnpm build
pnpm link --global
cd ../examples/typescript
pnpm link --global spatial-id-js
```

3. 開発サーバ起動

```sh
pnpm exec vite
```

4. ブラウザで http://localhost:5173/ を開き動作確認

---

- `spatial-id-js` をnpm公開した場合は、`pnpm add spatial-id-js` で直接依存可能です。
- 本サンプルは`vite.config.ts`でwasmバインディングも自動解決します。

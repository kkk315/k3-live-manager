# 運用ガイド

## 前提
- OS: Windows
- シェル: PowerShell 5.1

## 開発起動（フロントのみ）
```powershell
yarn dev
```

## Tauri 開発起動（フロント+Rust）
```powershell
yarn tauri dev
```

## ビルド
- フロントのみ: `yarn build`
- Tauriバンドル: `yarn tauri build`

## テスト
- Rust側ユニットテスト（ある場合）
```powershell
cargo test
```

## トラブルシュート
- ポート1421を他プロセスが使用していないか確認
- `app.sqlite` はプロジェクト直下（`sqlite:../app.sqlite` で参照）
- 秘密情報はログに出さない（トークン/クライアントシークレット）

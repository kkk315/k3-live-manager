# 仕様書: フロント `CredentialsListPage`

対象実装: `src/pages/CredentialsListPage.tsx`

## 概要

- 目的: 資格情報一覧を表示し、認証フロー開始ボタンを提供。
- URL: `/credentials`

## ユースケース

- アクター: ユーザー
- 事前条件: なし
- 基本フロー:
  - 画面表示時に `get_service_credentials` を呼び一覧表示
  - 行の「Authenticate」押下で `start_oauth_flow(credential_id)` を呼び出し、返却URLを外部ブラウザで開く
- 代替フロー/例外: 取得失敗/開始失敗時にアラート表示

## I/O 契約

- 入力: なし（初期ロードでinvoke）
- 出力: `ServiceCredential[]` の表示
- エラー: 失敗時はconsoleに記録しアラート

## 設計方針

- 依存: `@tauri-apps/api/core`, `@tauri-apps/plugin-opener`
- セキュリティ: 認可URLを開くのみ。stateはBEで管理し検証。

## テスト項目

- 正常系: 一覧0件/複数件表示、ボタンで既定ブラウザ起動
- 異常系: invoke失敗時のエラーハンドリング

 

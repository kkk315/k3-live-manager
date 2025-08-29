# 仕様書: フロント `AddCredentialPage`

対象実装: `src/pages/AddCredentialPage.tsx`

## 概要

- 目的: 新規のサービス資格情報を登録するフォーム。
- URL: `/credentials/add`

## ユースケース

- アクター: ユーザー
- 事前条件: なし
- 基本フロー:
  - 必須入力（service_name/client_id/client_secret）を埋める
  - 送信で `add_service_credential(payload)` を呼ぶ
  - 成功で `/credentials` に遷移
- 代替フロー/例外:
  - 未入力あり→エラー表示
  - invokeエラー→エラー表示

## I/O 契約

- 入力: `AddCredentialPayload`
- 出力: `ServiceCredential`（UIでは戻り値未使用）
- エラー: 文字列メッセージ

## 設計方針

- 依存: `@tauri-apps/api/core`, `react-router-dom`
- セキュリティ: client_secretはローカル保存だがログ出力禁止

## テスト項目

- 正常系: 正常登録→一覧へ遷移
- 異常系: 入力不足/エラー時のメッセージ表示

 

# 仕様書: ローカル OAuth コールバックサーバ

対象実装: `src-tauri/src/oauth_server.rs`

## 概要

- 目的: 固定ポート1421で1接続のみ受け付け、`/oauth/callback` で `code` と `state` を受け取り、oneshotで上位へ返却する。

## I/O 契約

`start_oauth_server(tx, port) -> anyhow::Result<()>`

- 入力: `tx: oneshot::Sender<(String, String)>`, `port: u16`
- 動作: 1回のHTTP/1.1接続を処理し、`/oauth/callback`訪問時に `(code,state)` を `tx` で送信
- 出力: `Ok(())`

## レスポンス仕様

- 成功時: 日本語の完了HTMLを返し、5秒後に自動クローズするJavaScriptを含む
- 失敗/その他パス: 404 Not Found

## 設計方針/セキュリティ

- 単一接続のみ（複数接続は想定しない）
- CSRF: stateの検証は上位（Command）で実施
- ログ: 機微情報を出力しない

## テスト項目

- 正常系: `/oauth/callback?code=...&state=...` で `(code,state)` が送出されHTMLが返る
- 例外系: クエリ欠落/その他パスで404

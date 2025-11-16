# Chess App — Built with Tauri

このアプリは **Rust + Tauri** を使用して構築されたデスクトップ向けチェスアプリです。  
軽量・高速なネイティブ実行と、シンプルな UI を両立した学習・対局ツールとして設計しています。

---

## 🎯 主な機能

- **チェスボードの描画**（HTML / CSS / JS）
- **合法手生成エンジン（Rust）**
- **Tauri を用いた高速な Rust <-> JS 通信**
- **ローカルで動作する軽量デスクトップアプリ**

---

## 🛠 技術スタック

### Frontend
- HTML
- CSS
- Vanilla JavaScript

### Backend (Core Logic)
- Rust  
- Bitboard を用いた高速な手生成

### Framework
- **Tauri**

---

## 🔧 開発環境のセットアップ

### 1. リポジトリを取得
```sh
git clone https://github.com/kamei28/chessv2.git
cd chessv2
```

### 2. Rustをインストール
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
公式: https://rust-lang.org/ja/tools/install/

### 3. TAURIをインストール
```sh
cargo install tauri-cli
```

### 4. 起動
```sh
cargo tauri dev
```
---

## 📝 ライセンス
MIT License  
Copyright (c) 2025 kamei28
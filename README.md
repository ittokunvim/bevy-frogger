# Bevy frogger

ゲームエンジンBevyで作られたフロッガー

## Wasmに変換する

ゲームをWasmに変換する場合は、以下のコマンドを実行します。

```sh
# ビルド
cargo build --release --target wasm32-unknown-unknown
# 変換
wasm-bindgen --target web --out-dir ./examples --no-typescript \
target/wasm32-unknown-unknown/release/ittoku_frogger.wasm
# 実行
npx http-server examples
```

## アセット

プレイヤーの画像は以下のURLからお借りしています。

https://obane.blog.shinobi.jp/charachip_animal/frog

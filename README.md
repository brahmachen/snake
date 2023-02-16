# snake
贪吃蛇游戏。

web版链接：[点这里](https://brahmachen.github.io/my-image-server/23-02/snake/)（电脑版Chrome/Firefox/Edge打开）

## 运行
1. 本地运行
```
cargo run
```
2. WASM运行
```
rustup target install wasm32-unknown-unknown
cargo install wasm-server-runner
cargo run --target wasm32-unknown-unknown
```
```
cargo install wasm-bindgen-cli
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/tetris.wasm
```

## 游戏展示
[demo视频](https://brahmachen.github.io/my-image-server/23-02/snake/demo.mp4)

## 参考资料
- [Rust Book](https://kaisery.github.io/trpl-zh-cn/title-page.html)
- [bevy github example](https://github.com/bevyengine/bevy)
- [bevy-cheatbook](https://github.com/bevy-cheatbook/bevy-cheatbook)（[中文翻译](https://yiviv.com/bevy-cheatbook/)）
- [俄罗斯方块游戏](https://github.com/NightsWatchGames/tetris)
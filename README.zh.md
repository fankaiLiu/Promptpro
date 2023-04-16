[英文版](README.md)
# promptpro

这是一个生成 AI 提示词的小工具,目前支持MidJourney的批量随机生成
请把config.toml放在和程序的统计目录下
## About

## Usage

1. 编译构建
   ```
   cargo build
   ```
2. 运行
   ```
   cargo run
   ```

以wasm方式运行
```
cargo install wasm-pack
//
asm-pack build --target web --dev
//或者wasm-pack build --release --target web
//然后
python3 -m http.server

```
在http://localhost:8000访问页面
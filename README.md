# tauri-learn-202210
Tauri学习

# 要点记录
- tauri.conf.json 中的 tauri.windows.x 与 tauri.windows.y 须成对出现方可生效

# 创建工程

- pnpm create tauri-app // 安装  tauri-app
```
 1.输入项目名称 test5

 2.选择使用pnpm

 3.选择vue  // 不要选择vue-ts,其使用的ts版本尚不兼容当前的某些规范，导致 vue 文件 中 onclick 事件报错

 4. pnpm install
 
 5. cd src-tauri &&  CARGO_NET_GIT_FETCH_WITH_CLI=true cargo run  // 下载cargo中依赖的包，可忽略个别文件编译不通过

 6. cd 项目根目录 pnpm tauri build
 6.1  若要支持windows7 
        carog.toml 中添加
        [dependencies]
        tauri = { version = "1", features = [ "windows7-compat" ] }
 7.src-tauri/target/release/bundle/      //打包出的安装文件位置

 --- 8.  upx --ultra-brute src-tauri/target/release/bundle/macos/test5.app/Contents/macOS/test5           // upx再次压缩,但是压缩后就报 您没有权限来打开应用程序，

 9. cd 项目根目录 CARGO_NET_GIT_FETCH_WITH_CLI=true  pnpm tauri-dev  // 开发模式启动项目
 

```

# elf 
``` shell
cd go-for-elf

chmod +x build.py

python3 ./build.py 
```

# 安全策略
- https://tauri.app/v1/api/js/tauri

# 构建指定系统应用
pnpm tauri build --target aarch64-apple-darwin

pnpm tauri build --target x86_64-apple-darwin

pnpm tauri build --target universal-apple-darwin

# cargo.toml 参数
opt-level 支持的选项包括:

0: 无优化
1: 基本优化
2: 一些优化
3: 全部优化
"s": 优化输出的二进制文件的大小
"z": 优化二进制文件大小，但也会关闭循环向量化

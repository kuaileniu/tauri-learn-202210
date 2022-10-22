# tauri-learn-202210
Tauri学习

# 创建工程

- pnpm create tauri-app // 安装  tauri-app
```
 1.输入项目名称 test5

 2.选择使用pnpm

 3.选择vue  // 不要选择vue-ts,其使用的ts版本尚不兼容当前的某些规范，导致 vue 文件 中 onclick 事件报错

 4.pnpm install
 
 5. cd src-tauri &&  CARGO_NET_GIT_FETCH_WITH_CLI=true cargo run  // 下载cargo中依赖的包，可忽略个别文件编译不通过

 6. cd 项目根目录 pnpm tauri build

 7.src-tauri/target/release/bundle/      //打包出的安装文件位置

 --- 8.  upx --ultra-brute src-tauri/target/release/bundle/macos/test5.app/Contents/macOS/test5           // upx再次压缩,但是压缩后就报 您没有权限来打开应用程序，

 9. cd 项目根目录 pnpm tauri-dev  // 开发模式启动项目
 

```
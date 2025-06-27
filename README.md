# Terminal Image Viewer

## 概要

各種画像を閲覧する機能

* 64x64 PNG Image
    * RGBA (フルカラー) は、256ビットカラーに変換

* 表示パターン
* 表示パターン素材
* 表示パターン素材画像


## ビルド

### ターゲット用のRust標準ライブラリをインストール

#### Linux ARMv7l（armv7-unknown-linux-gnueabihf）環境向け

```sh
rustup target add armv7-unknown-linux-gnueabihf
```

#### クロスコンパイラ（リンカ）のインストール

```sh
brew tap messense/macos-cross-toolchains
brew install armv7-unknown-linux-gnueabihf
```

インストールの確認

```zsh
% armv7-unknown-linux-gnueabihf-gcc --version       
zsh: command not found: arm-linux-gnueabihf-gcc
% brew --prefix armv7-unknown-linux-gnueabihf
/usr/local/opt/armv7-unknown-linux-gnueabihf
% export PATH="/usr/local/opt/armv7-unknown-linux-gnueabihf/bin:$PATH"
 % armv7-unknown-linux-gnueabihf-gcc --version
armv7-unknown-linux-gnueabihf-gcc (GCC) 13.3.0
Copyright (C) 2023 Free Software Foundation, Inc.
This is free software; see the source for copying conditions.  There is NO
warranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.

```

```
% rustup target list --installed
armv7-unknown-linux-gnueabihf
x86_64-apple-darwin
```

#### クロスコンパイラのターゲット設定(追加) .cargo/config.toml

```
[target.armv7-unknown-linux-gnueabihf]
linker = "armv7-unknown-linux-gnueabihf-gcc"
```

#### クロスコンパイル

```sh
cargo build --target armv7-unknown-linux-gnueabihf --release
```

生成先
```
target/armv7-unknown-linux-gnueabihf/release/dpv
```

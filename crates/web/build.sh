#!/usr/bin/env bash
set -eux

# 安装 Rust 工具链
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

# 添加wasm32-unknown-unknown目标
rustup target add wasm32-unknown-unknown

# 安装 trunk
cargo install trunk

# 构建
trunk build --release

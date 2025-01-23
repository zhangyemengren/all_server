#!/bin/bash
git pull

# 编译项目
cargo build --release --package hs_server
cargo build --release --package book_server

# 复制 systemd 服务文件
sudo cp docker-app.service /etc/systemd/system/
sudo cp hs.service /etc/systemd/system/
sudo cp rs.service /etc/systemd/system/

# 重新加载 systemd 配置
sudo systemctl daemon-reload

# 设置开机自启
sudo systemctl enable docker-app
sudo systemctl enable hs
sudo systemctl enable rs

# 启动服务
sudo systemctl start docker-app
sudo systemctl start hs
sudo systemctl start rs



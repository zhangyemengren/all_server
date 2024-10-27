FROM rust:bookworm as builder
WORKDIR /usr/src/app
COPY . .
RUN apt-get update && apt-get install -y git
RUN cargo install trunk
# 进入 web 文件夹并打包前端资源
WORKDIR /usr/src/app/web
RUN trunk build --release
# 移动打包后的资源到 dist 目录
RUN mkdir -p /usr/src/app/dist && \
    mv dist/* /usr/src/app/dist/ \
# 回到根目录并继续构建 server
WORKDIR /usr/src/app
RUN cargo build --release --package server --bin server

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/server /usr/local/bin/hs_server

EXPOSE 8000
CMD ["hs_server"]

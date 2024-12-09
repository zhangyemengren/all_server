# hs_server 使用 systemd 管理

#FROM rust:bookworm as builder
#WORKDIR /usr/src/app
#COPY . .
#RUN cargo build --release --package hs_server --bin hs_server
#
#FROM debian:bookworm-slim
#RUN apt-get update && apt-get install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*
#COPY --from=builder /usr/src/app/target/release/hs_server /usr/local/bin/hs_server
#
#EXPOSE 8000
#CMD ["hs_server"]

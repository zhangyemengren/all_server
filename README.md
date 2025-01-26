#

## 开发相关
### db变化
- sqlx database create 通过.env中的相关变量创建数据库
- sqlx migrate add <name> 添加条目
## 发布相关
参考deploy.sh

## 本地/x线上 环境
### orbstack/docker
- cd 项目目录
- docker compose up -d
### .env key
client_id=
client_secret=
DATABASE_URL=
POSTGRES_USER=
POSTGRES_PASSWORD=
POSTGRES_DB=
### db migrate
- cargo install sqlx-cli
- sqlx database create 通过.env中的相关变量创建数据库
- sqlx migrate run 迁移

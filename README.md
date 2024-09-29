## roadmap
### server
### c端
标准卡牌
酒馆战棋
资讯
我的(自定义卡组，收藏卡组，历史记录)
### b端
上传数据
## api 

### get token
curl -u {client_id}:{client_secret} -d grant_type=client_credentials https://oauth.battle.net/token

### api with token
curl -H "Authorization: Bearer {}" "https://us.api.blizzard.com/hearthstone/cards/52119-arch-villain-rafaam?locale=zh_CN"

## docker运行时指定配置文件
sudo docker build -t hs_server .
sudo docker compose up

## ENV KEY
client_id=
client_secret=
DATABASE_URL=
POSTGRES_USER=
POSTGRES_PASSWORD=
POSTGRES_DB=

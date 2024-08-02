## api 

### get token
curl -u {client_id}:{client_secret} -d grant_ty
pe=client_credentials https://oauth.battlenet.com.cn/token

### api with token
curl -H "Authorization: Bearer {}" "https://us.api.blizza
rd.com/hearthstone/cards/52119-arch-villain-rafaam?locale=zh_CN"
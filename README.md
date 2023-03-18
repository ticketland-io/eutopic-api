# eutopic-api

# Build docker image

```bash
docker build --build-arg GITHUB_TOKEN=<github_token> -t registry.digitalocean.com/ticketland/eutopic-api:<version> -f ./operations/api/Dockerfile ./src/api
```


# Create symetric key

We add an `ENC_KEY` env variable. This is an AES 256 encryption key

`openssl enc -aes-256-gcm -k secret -P -md sha1`

This emits something like this

```
salt=3F9281C9D2E90157
key=EBF6A062A9DE7B4FCE764BCE671CC14AA064EB57AA69E9E5135241F501A14B50
iv =4BD321E45194A19508367AA4
```

Copy `key` value and use it as `ENC_KEY`

# eutopic-api

# Build docker image

```bash
docker build --build-arg GITHUB_TOKEN=<github_token> -t registry.digitalocean.com/ticketland/eutopic-api:<version> -f ./operations/api/Dockerfile ./src/api
```

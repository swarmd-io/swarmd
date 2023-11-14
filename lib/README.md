# Update API from Client

```
wget https://api.swarmd.io/docs/private/api.json
wget http://localhost:8087/docs/private/api.json
openapi-generator generate -i http://localhost:8087/docs/private/api.json -g rust -o swarmd-generated
rm api.json
```

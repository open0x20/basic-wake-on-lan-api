# basic-wake-on-lan-api



## Example request:

```bash
curl --location --request POST '127.0.0.1:8080/wake' \
--header 'Content-Type: application/json' \
--data-raw '{
    "mac_address": "44:FE:70:1F:9A:1C"
}'
```


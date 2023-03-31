# api-db-rs

Simple API in rust to connecto to a database

## requests

### read all records

```
curl -i \
-H "content-type: application/json" \
-X GET http://localhost:8080/records
```

### read record by id

```
curl -i \
-H "content-type: application/json" \
-X GET http://localhost:8080/records/1
```

### create record

```
curl -i \
-H "content-type: application/json" \
-X POST http://localhost:8080/records \
-d '{"name":"test"}'
```

### update record

```
curl -i \
-H "content-type: application/json" \
-X PATCH http://localhost:8080/records/1 \
-d '{"name":"update name"}'
```

### delete record

```
curl -i \
-H "content-type: application/json" \
-X DELETE http://localhost:8080/records/1
```

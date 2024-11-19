TAG="latest"

SERVICE="api-db-rs"

REPO="memotoro"

.PHONY: default
default: local-run

.PHONY: local-run
local-run:
	cargo clippy
	cargo run

.PHONY: db
db:
	docker compose up db -d

.PHONY: up
up:
	docker compose up --build

.PHONY: down
down:
	docker compose down --volumes

.PHONY: docker-image
docker-image:
	docker build -t $(REPO)/$(SERVICE):$(TAG) -f Dockerfile .

.PHONY: docker-push
docker-push:
	docker push $(REPO)/$(SERVICE):$(TAG)

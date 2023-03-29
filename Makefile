.PHONY: default
default: run

.PHONY: run
run:
	cargo clippy
	cargo run

.PHONY: up
up:
	docker compose up -d 

.PHONY: down
down:
	docker compose down --volumes

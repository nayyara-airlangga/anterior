run:
	cargo run

watch:
	cargo watch -x run

up:
	docker compose up -d

down:
	docker compose down

logs:
	docker compose logs -f

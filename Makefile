run:
	cargo run

watch:
	cargo watch -x run

dev-up:
	docker compose up -d

dev-down:
	docker compose down

logs:
	docker compose logs -f

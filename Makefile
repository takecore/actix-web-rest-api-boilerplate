.PHONY: run
run:
	cargo run

.PHONY: migrate
migrate:
	diesel migration run

.PHONY: migrate-redo
migrate-redo:
	diesel migration redo
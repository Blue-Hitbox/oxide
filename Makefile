.PHONY: all build-backend build-frontend run-backend run-frontend clean help

all: build-backend build-frontend

build-backend:
	cargo build -p backend

build-frontend:
	rustup target add wasm32-unknown-unknown
	cd frontend && trunk build || cargo build -p frontend --target wasm32-unknown-unknown

run-backend:
	cargo run -p backend

run-frontend:
	cd frontend && trunk serve || (echo "Please install trunk to run the frontend server: cargo install trunk" && exit 1)

clean:
	cargo clean

help:
	@echo "Available commands:"
	@echo "  make build-backend  - Build the backend crate"
	@echo "  make build-frontend - Build the frontend crate"
	@echo "  make run-backend    - Run the backend server"
	@echo "  make run-frontend   - Run the frontend development server (requires trunk)"
	@echo "  make clean          - Clean the workspace"

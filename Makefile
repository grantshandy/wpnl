all: frontend release

frontend:
	cd web && npm run build && cd ..
	
release: frontend
	cargo b --release
	
run: frontend
	cargo r
	
dev:
	cargo watch -s "make run"
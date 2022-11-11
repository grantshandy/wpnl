all: web release

web:
	cd web && npm run build && cd ..
	
release: web
	cargo b --release
	
run: web
	cargo r
	
dev:
	cargo watch -s "make run"
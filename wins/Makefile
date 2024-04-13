test: 
	cargo test
lint:
	cargo clippy --no-deps -- -D warnings
docker-build:
	docker build -t wins .
docker-run:
	docker run -p 3000:3000 --rm -it wins 

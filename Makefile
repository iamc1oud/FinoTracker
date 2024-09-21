dev:
	cargo watch -w src/ -x  run

start:
	cargo run

mongo_init:
	docker run --name fino_db -p 27017:27017 -d mongodb/mongodb-community-server:latest
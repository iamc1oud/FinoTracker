test:
	uv run pytest

build:
	docker build -t auth-service:latest .

db-forward:
	kubectl port-forward svc/auth-postgres 5432:5432

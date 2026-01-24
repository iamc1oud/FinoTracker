FROM python:3.13-slim

WORKDIR /app
COPY pyproject.toml ./
RUN pip install --upgrade pip && pip install "uv" && pip install .

COPY ./app ./app
COPY ./alembic.ini ./alembic.ini
COPY ./migrations ./migrations

CMD ["uvicorn", "app.main:app", "--host", "0.0.0.0", "--port", "8000"]

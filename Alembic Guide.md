Perfect! Let’s focus on **wiring the auth-service to the API Gateway** first, and also integrate **PostgreSQL with proper migrations** so you don’t rely on `Base.metadata.create_all`. I’ll guide step by step.

---

# 🔹 Step 4 — Auth-Service: Postgres + Migrations + API Gateway Wiring

---

## 1️⃣ Replace SQLite with PostgreSQL

Update `database.py`:

```python
# app/database.py
from sqlalchemy import create_engine
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import sessionmaker
import os

POSTGRES_USER = os.environ.get("POSTGRES_USER", "postgres")
POSTGRES_PASSWORD = os.environ.get("POSTGRES_PASSWORD", "postgres")
POSTGRES_DB = os.environ.get("POSTGRES_DB", "authdb")
POSTGRES_HOST = os.environ.get("POSTGRES_HOST", "localhost")
POSTGRES_PORT = os.environ.get("POSTGRES_PORT", "5432")

SQLALCHEMY_DATABASE_URL = f"postgresql://{POSTGRES_USER}:{POSTGRES_PASSWORD}@{POSTGRES_HOST}:{POSTGRES_PORT}/{POSTGRES_DB}"

engine = create_engine(SQLALCHEMY_DATABASE_URL, pool_pre_ping=True)
SessionLocal = sessionmaker(bind=engine, autoflush=False, autocommit=False)
Base = declarative_base()
```

---

## 2️⃣ Add Alembic for migrations

### Install Alembic

```bash
pip install alembic psycopg[binary]
```

### Initialize Alembic

```bash
alembic init migrations
```

* `alembic.ini` → update `sqlalchemy.url` to match `SQLALCHEMY_DATABASE_URL`
* `env.py` → import your models:

```python
from app.models import Base
target_metadata = Base.metadata
```

---

### Generate initial migration

```bash
alembic revision --autogenerate -m "create users table"
alembic upgrade head
```

✅ Now tables are created via migrations, not on runtime.

---

## 3️⃣ API Gateway routing → auth-service

Assuming your **API Gateway** is a FastAPI app with **httpx for proxying**:

```python
# services/api-gateway/app/routes/auth_proxy.py
from fastapi import APIRouter, Request
import httpx
import os

router = APIRouter()
AUTH_SERVICE_URL = os.environ.get("AUTH_SERVICE_URL", "http://auth-service:80")

@router.api_route("/auth/{path:path}", methods=["GET", "POST", "PUT", "DELETE"])
async def proxy_auth(request: Request, path: str):
    async with httpx.AsyncClient() as client:
        url = f"{AUTH_SERVICE_URL}/{path}"
        resp = await client.request(
            request.method,
            url,
            headers=request.headers.raw,
            content=await request.body()
        )
    return resp.json()
```

* Any `/auth/*` request to API Gateway will **forward to auth-service**
* Use `AUTH_SERVICE_URL=http://auth-service:80` (K8s service name)

---

## 4️⃣ K8s Deployment: Postgres + Auth-Service

### Postgres deployment

```yaml
apiVersion: v1
kind: Secret
metadata:
  name: auth-postgres-secret
type: Opaque
stringData:
  POSTGRES_USER: "postgres"
  POSTGRES_PASSWORD: "postgres"

---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: auth-postgres
spec:
  replicas: 1
  selector:
    matchLabels:
      app: auth-postgres
  template:
    metadata:
      labels:
        app: auth-postgres
    spec:
      containers:
      - name: postgres
        image: postgres:15
        envFrom:
          - secretRef:
              name: auth-postgres-secret
        ports:
          - containerPort: 5432
        volumeMounts:
          - mountPath: /var/lib/postgresql/data
            name: postgres-data
      volumes:
        - name: postgres-data
          emptyDir: {}
---
apiVersion: v1
kind: Service
metadata:
  name: auth-postgres
spec:
  selector:
    app: auth-postgres
  ports:
    - port: 5432
```

### Auth-Service deployment (env updated)

```yaml
env:
  - name: POSTGRES_HOST
    value: auth-postgres
  - name: POSTGRES_DB
    value: authdb
  - name: POSTGRES_USER
    valueFrom:
      secretKeyRef:
        name: auth-postgres-secret
        key: POSTGRES_USER
  - name: POSTGRES_PASSWORD
    valueFrom:
      secretKeyRef:
        name: auth-postgres-secret
        key: POSTGRES_PASSWORD
```

---

## 5️⃣ Run Alembic inside K8s

* Either run a **job** in K8s to migrate:

```yaml
apiVersion: batch/v1
kind: Job
metadata:
  name: auth-migrate
spec:
  template:
    spec:
      containers:
      - name: migrate
        image: auth-service:latest
        command: ["alembic", "upgrade", "head"]
        envFrom:
          - secretRef:
              name: auth-postgres-secret
      restartPolicy: OnFailure
```

✅ Ensures migrations run **before service starts**.

---

## ✅ Outcome

* **Auth-service** uses **Postgres + Alembic migrations**
* API Gateway forwards `/auth/*` routes to auth-service
* Fully IoC with `python-dependency-injector`
* Secure, TDD-ready, scalable

---

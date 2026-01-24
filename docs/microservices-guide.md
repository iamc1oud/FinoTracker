# Microservices Communication Guide

## Overview

This guide covers implementing microservice communication in FinoTracker using **FastStream** - a Python framework similar to NestJS microservices module that supports Kafka, NATS, RabbitMQ, and Redis.

## Architecture

```
                    ┌─────────────────┐
                    │     Client      │
                    └────────┬────────┘
                             │
                    ┌────────▼────────┐
                    │   API Gateway   │
                    │   (FastAPI)     │
                    └────────┬────────┘
                             │
              ┌──────────────┼──────────────┐
              │              │              │
     ┌────────▼───┐  ┌───────▼────┐  ┌──────▼──────┐
     │auth-service│  │user-service│  │payment-svc  │
     └────────┬───┘  └──────┬─────┘  └──────┬──────┘
              │             │               │
              └─────────────┼───────────────┘
                            │
                   ┌────────▼────────┐
                   │   NATS / Kafka  │
                   │  Message Broker │
                   └─────────────────┘
```

## Why FastStream?

FastStream provides:
- Unified API for multiple brokers (Kafka, NATS, RabbitMQ, Redis)
- Pydantic validation built-in
- Automatic AsyncAPI documentation
- Dependency injection system
- Easy integration with FastAPI
- In-memory testing support

## Installation

```bash
# For NATS (lightweight, recommended for starting)
uv add "faststream[nats]"

# For Kafka (production-scale)
uv add "faststream[kafka]"

# For RabbitMQ
uv add "faststream[rabbit]"
```

## Broker Comparison

| Broker | Best For | Complexity |
|--------|----------|------------|
| NATS | Simple pub/sub, low latency | Low |
| Kafka | High throughput, event sourcing, audit logs | High |
| RabbitMQ | Complex routing, reliability | Medium |
| Redis | Simple caching + pub/sub | Low |

**Recommendation**: Start with NATS for simplicity, migrate to Kafka when you need event sourcing or high throughput.

---

## Implementation

### Step 1: Deploy NATS in Kubernetes

Create `infra/k8s/base/nats.yaml`:

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: nats
spec:
  replicas: 1
  selector:
    matchLabels:
      app: nats
  template:
    metadata:
      labels:
        app: nats
    spec:
      containers:
        - name: nats
          image: nats:latest
          ports:
            - containerPort: 4222

---
apiVersion: v1
kind: Service
metadata:
  name: nats
spec:
  selector:
    app: nats
  ports:
    - port: 4222
      targetPort: 4222
  type: ClusterIP
```

### Step 2: Auth Service - Publisher & Subscriber

`services/auth-service/app/broker.py`:

```python
from faststream import FastStream
from faststream.nats import NatsBroker
import os

NATS_URL = os.environ.get("NATS_URL", "nats://nats:4222")

broker = NatsBroker(NATS_URL)
app = FastStream(broker)
```

`services/auth-service/app/events/schemas.py`:

```python
from pydantic import BaseModel, EmailStr
from uuid import UUID


class UserCreated(BaseModel):
    user_id: UUID
    email: EmailStr


class UserLoggedIn(BaseModel):
    user_id: UUID
    email: EmailStr
```

`services/auth-service/app/events/publishers.py`:

```python
from app.broker import broker
from app.events.schemas import UserCreated, UserLoggedIn


async def publish_user_created(user_id: UUID, email: str):
    await broker.publish(
        UserCreated(user_id=user_id, email=email),
        subject="user.created"
    )


async def publish_user_logged_in(user_id: UUID, email: str):
    await broker.publish(
        UserLoggedIn(user_id=user_id, email=email),
        subject="user.logged_in"
    )
```

`services/auth-service/app/events/subscribers.py`:

```python
from app.broker import broker
from app.events.schemas import UserCreated


@broker.subscriber("user.created")
async def handle_user_created(event: UserCreated):
    """Handle user created event (e.g., send welcome email)"""
    print(f"User created: {event.email}")
```

### Step 3: Integrate with FastAPI

`services/auth-service/app/main.py`:

```python
from contextlib import asynccontextmanager
from fastapi import FastAPI
from app.broker import broker
from app.routes import auth


@asynccontextmanager
async def lifespan(app: FastAPI):
    await broker.start()
    yield
    await broker.close()


app = FastAPI(lifespan=lifespan)
app.include_router(auth.router)
```

### Step 4: Publish Events from Routes

`services/auth-service/app/routes/auth.py`:

```python
from fastapi import APIRouter, Depends
from app.events.publishers import publish_user_created
from app.schemas.schemas import CreateUser

router = APIRouter(prefix="/auth", tags=["auth"])


@router.post("/register")
async def register(user: CreateUser):
    # ... create user in database ...
    
    # Publish event
    await publish_user_created(user_id=new_user.id, email=new_user.email)
    
    return {"message": "User registered"}
```

### Step 5: API Gateway - Request/Response Pattern

For synchronous request/response through the gateway:

`services/api-gateway/app/broker.py`:

```python
from faststream.nats import NatsBroker
import os

NATS_URL = os.environ.get("NATS_URL", "nats://nats:4222")
broker = NatsBroker(NATS_URL)
```

`services/api-gateway/app/routes/auth.py`:

```python
from fastapi import APIRouter, HTTPException
from app.broker import broker
from pydantic import BaseModel

router = APIRouter(prefix="/auth", tags=["auth"])


class LoginRequest(BaseModel):
    email: str
    password: str


class LoginResponse(BaseModel):
    access_token: str
    user_id: str


@router.post("/login", response_model=LoginResponse)
async def login(request: LoginRequest):
    # Request/Response pattern via NATS
    response = await broker.request(
        request,
        subject="auth.login",
        timeout=5.0
    )
    
    if response is None:
        raise HTTPException(status_code=503, detail="Auth service unavailable")
    
    return response
```

Auth service handler:

`services/auth-service/app/events/handlers.py`:

```python
from app.broker import broker
from pydantic import BaseModel


class LoginRequest(BaseModel):
    email: str
    password: str


class LoginResponse(BaseModel):
    access_token: str
    user_id: str


@broker.subscriber("auth.login")
async def handle_login(request: LoginRequest) -> LoginResponse:
    # Validate credentials, generate token
    # ... authentication logic ...
    
    return LoginResponse(
        access_token="jwt_token_here",
        user_id="user_uuid_here"
    )
```

---

## Communication Patterns

### 1. Publish/Subscribe (Event-Driven)

One service publishes, multiple services can subscribe.

```python
# Publisher
await broker.publish(UserCreated(...), subject="user.created")

# Subscriber (any service)
@broker.subscriber("user.created")
async def handle(event: UserCreated):
    ...
```

**Use for**: Notifications, audit logs, cache invalidation

### 2. Request/Response (RPC)

Synchronous request expecting a response.

```python
# Requester (API Gateway)
response = await broker.request(data, subject="auth.login", timeout=5.0)

# Responder (Auth Service)
@broker.subscriber("auth.login")
async def handle(request: LoginRequest) -> LoginResponse:
    return LoginResponse(...)
```

**Use for**: API calls that need immediate response

### 3. Queue Groups (Load Balancing)

Multiple instances of same service, only one handles message.

```python
@broker.subscriber("orders.process", queue="order-workers")
async def handle(order: Order):
    ...
```

**Use for**: Distributed processing, scaling workers

---

## Kubernetes Deployment

Update `infra/k8s/base/deployment.yaml` to add NATS_URL:

```yaml
env:
  - name: NATS_URL
    value: "nats://nats:4222"
```

---

## Testing

FastStream provides in-memory testing:

```python
import pytest
from faststream.nats import TestNatsBroker
from app.broker import broker
from app.events.subscribers import handle_user_created


@pytest.mark.asyncio
async def test_user_created_handler():
    async with TestNatsBroker(broker) as br:
        await br.publish(
            {"user_id": "123", "email": "test@example.com"},
            subject="user.created"
        )
        
        # Assert handler was called
        handle_user_created.mock.assert_called_once()
```

---

## Migration Path

1. **Phase 1**: HTTP proxying (current)
2. **Phase 2**: Add NATS for async events (user.created, etc.)
3. **Phase 3**: Move to request/response for service-to-service calls
4. **Phase 4**: Migrate to Kafka if you need event sourcing/replay

---

## Resources

- [FastStream Documentation](https://faststream.airt.ai/)
- [FastStream GitHub](https://github.com/ag2ai/faststream)
- [NATS Documentation](https://nats.io/)

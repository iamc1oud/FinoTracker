from fastapi import FastAPI
from app.routes.health import router as health_router
import logging

logging.basicConfig(level=logging.INFO)

app = FastAPI(title="AI Expense Manager API Gateway")
app.include_router(health_router, prefix="/health")

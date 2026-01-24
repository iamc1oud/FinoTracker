from fastapi import FastAPI
from app.routes import auth
from app.ioc import Container
from app.database import Base, engine

Base.metadata.create_all(bind=engine)

container = Container()

container.wire(modules=[auth])

app = FastAPI()

app.include_router(auth.router, prefix="/auth", tags=["Authentication"])

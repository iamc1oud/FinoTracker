from dependency_injector import containers, providers
from sqlalchemy.orm import Session
from .database import SessionLocal
from .services.auth_service import AuthService

class Container(containers.DeclarativeContainer):
    wiring_config = containers.WiringConfiguration(modules=[".routes.auth"])

    # DB session provider (per request)
    db_session = providers.Factory(SessionLocal)

    # AuthService provider (injects DB Session)
    auth_service = providers.Factory(AuthService, db=db_session)

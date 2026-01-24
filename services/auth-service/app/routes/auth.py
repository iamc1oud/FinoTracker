from fastapi.responses import JSONResponse
from app.ioc import Container
from app.services.auth_service import AuthService
from app.schemas.schemas import CreateUser
from dependency_injector.wiring import inject, Provide
from fastapi import APIRouter, Depends, HTTPException

router = APIRouter()

@router.post("/register")
@inject
def register(
    user: CreateUser,
    auth_service: AuthService = Depends(Provide[Container.auth_service])
):
    try:
        new_user = auth_service.register_user(user_data=user)

        return JSONResponse(
            status_code=201,
            content={"status": "ok"}
        )
    except ValueError as e:
        raise HTTPException(status_code=400, detail=str(e))

# TODO: Add other endpoints

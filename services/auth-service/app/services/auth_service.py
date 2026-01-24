from app.schemas.schemas import CreateUser
from sqlalchemy.orm import Session

class AuthService:
    def __init__(self, db: Session):
        """
        Initialize the AuthService with a database session.

        Args:
            db (Session): The database session to use for database operations.
        """
        self.db = db

    def register_user(self, user_data: CreateUser):
        # TODO: Add DB logic
        return user_data

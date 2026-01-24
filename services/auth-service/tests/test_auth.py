from app.database import Base, engine
from fastapi.testclient import TestClient
import pytest
from app.main import app

client = TestClient(app)

@pytest.fixture(scope="module", autouse=True)
def setup_db():
    Base.metadata.drop_all(bind=engine)
    Base.metadata.create_all(bind=engine)
    yield
    Base.metadata.drop_all(bind=engine)

def test_register():
    res = client.post("/auth/register", json={"email": "test@example.com", "password": "password"})
    assert res.status_code == 201
    assert res.json()["status"] == "ok"

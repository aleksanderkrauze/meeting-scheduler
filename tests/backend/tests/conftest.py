import pytest


@pytest.fixture
def server_address() -> str:
    return "localhost:4444"

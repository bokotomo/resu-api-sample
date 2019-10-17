import pytest
import requests
from e2e.config import host


def test_health():
    url = host + "health"
    r = requests.get(url)
    assert r.status_code == 200

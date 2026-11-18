"""Foken API client."""

import requests
from typing import Dict

class FokenClient:
    def __init__(self, base_url: str = "http://localhost:8080"):
        self.base_url = base_url
    
    def health(self) -> Dict:
        """Check health."""
        return requests.get(f"{self.base_url}/health").json()

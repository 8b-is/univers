"""NVIDIA API client."""

import os
import requests
from typing import List, Optional

class NvidiaClient:
    def __init__(self, api_key: Optional[str] = None):
        self.api_key = api_key or os.getenv("NVIDIA_API_KEY")
        self.base_url = "https://integrate.api.nvidia.com/v1"
    
    def list_models(self) -> List[str]:
        """List available models."""
        if not self.api_key:
            return []
        headers = {"Authorization": f"Bearer {self.api_key}"}
        resp = requests.get(f"{self.base_url}/models", headers=headers)
        return [m["id"] for m in resp.json().get("data", [])]

"""NVIDIA model management."""

from typing import List

class ModelManager:
    def __init__(self):
        self.models = []
    
    def load_models(self) -> List[str]:
        """Load available models."""
        return self.models

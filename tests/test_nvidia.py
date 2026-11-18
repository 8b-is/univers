"""Test NVIDIA integration."""

import sys
sys.path.insert(0, "../src")

from integrations.nvidia import NvidiaClient

def test_list_models():
    client = NvidiaClient()
    models = client.list_models()
    print(f"Found {len(models)} models")
    assert len(models) > 0

if __name__ == "__main__":
    test_list_models()

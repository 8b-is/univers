# Univers

NVIDIA Omniverse + Smart Tree + Foken integration.

## Components

- **NVIDIA Integration**: API client for NVIDIA NIM
- **Smart Tree**: Tree management for code understanding
- **Foken**: AI-powered code analysis

## Quick Start

```bash
# Build Rust core
cargo build --release

# Run Python tests
python tests/test_nvidia.py
```

## Architecture

```
univers/
├── src/
│   ├── core/           # Core functionality
│   ├── integrations/   # External integrations
│   │   ├── nvidia/     # NVIDIA NIM
│   │   ├── smart-tree/ # Tree management
│   │   └── foken/      # Foken AI
│   └── cli/            # Command line interface
└── tests/              # Test suite
```

## License

MIT

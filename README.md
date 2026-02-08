# Neural Network Arena

Competitive neural network evolution platform with a custom virtual machine, implemented in Rust and compiled to WebAssembly for a web UI.

## Structure

- `src/` - Core Rust engine and VM
- `www/` - Web frontend wrapper for WASM
- `benches/` - Criterion benchmarks
- `tests/` - Rust tests

## Current Status

- Rust crate and WASM bindings are present.
- Web UI exists in `www`, but runtime behavior not verified in this audit.
- Operational estimate: **40%** (core engine scaffold, unverified integration).

## API Endpoints

- None. This is a Rust/WASM library with a browser UI.

## Tests

- Rust tests and benches exist but were not run (avoided long Rust builds).

## Future Work

- Validate WASM build pipeline and browser runtime.
- Add integration tests for VM and evolution logic.
- Document public API for embedding and experimentation.

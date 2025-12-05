## Python examples

### Build and run

```sh
# From repo root
cargo build --release

# Run examples
python3 examples/Python/basic.py
python3 examples/Python/self_print.py
```

Notes: Uses Python's `ctypes` to load `libbat_c.(dylib|so|dll)` from `target/release`. The examples mirror the C versions using `bat_pretty_print_to_string()` and `bat_free_string()`.

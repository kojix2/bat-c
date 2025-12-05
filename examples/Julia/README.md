## Julia examples

### Build and run

```sh
# From repo root
cargo build --release

# Run examples
julia examples/Julia/basic.jl
julia examples/Julia/self_print.jl
```

Notes: Uses Julia's `ccall` to load and call functions from `libbat_c.(dylib|so|dll)` in `target/release`. The examples mirror the C versions using `bat_pretty_print_to_string()` and `bat_free_string()`.

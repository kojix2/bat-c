## Go examples

### Build and run (dynamic linking)

```sh
cd examples/Go
make
make run-basic
make run-self_print
```

### Static linking

```sh
cd examples/Go
# Build static library if needed
cargo build --manifest-path ../../Cargo.toml --release
make static-basic
make static-self_print
./basic-static
./self_print-static
```

Notes: Uses cgo to call `bat_pretty_print_to_string()` and `bat_free_string()`. The Makefile embeds rpath for dynamic builds. For static builds, ensure `libbat_c.a` is available in `target/release` (requires staticlib in Cargo.toml).

## Odin examples

### Build and run (dynamic linking)

```sh
# From repo root
cargo build --release

cd examples/Odin
# macOS
odin run basic.odin -file
DYLD_LIBRARY_PATH=../../target/release odin run self_print.odin -file

# Linux (modify .odin files to use .so instead of .dylib)
odin run basic.odin -file
LD_LIBRARY_PATH=../../target/release odin run self_print.odin -file
```

### Static linking

```sh
# Build static library
cargo build --manifest-path ../../Cargo.toml --release

# Modify .odin files to link statically with libbat_c.a, then:
odin build basic.odin -file
odin build self_print.odin -file
./basic
./self_print
```

Notes: Uses Odin's `foreign import` to load `libbat_c.(dylib|so)`. The examples need the library path in `foreign import` statement adjusted for your platform. For static builds, ensure `libbat_c.a` is available in `target/release`.

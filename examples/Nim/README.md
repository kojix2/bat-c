## Nim examples

### Build and run (dynamic linking)

```sh
# From repo root
cargo build --release

cd examples/Nim
nim c -r basic.nim
nim c -r self_print.nim
```

### Static linking

```sh
# Build static library
cargo build --manifest-path ../../Cargo.toml --release

# Modify .nim files to link statically, then:
nim c --passL:"-static" -r basic.nim
nim c --passL:"-static" -r self_print.nim
```

Notes: Uses Nim's `{.importc.}` pragma to import C functions from `bat.h`. The examples embed rpath in the compiled binaries. For static builds, ensure `libbat_c.a` is available in `target/release`.

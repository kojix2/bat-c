## Zig examples

### Build and run (dynamic linking)

```sh
# From repo root
cargo build --release

cd examples/Zig
zig build
DYLD_LIBRARY_PATH=../../target/release ./zig-out/bin/basic      # macOS
LD_LIBRARY_PATH=../../target/release ./zig-out/bin/basic        # Linux
DYLD_LIBRARY_PATH=../../target/release ./zig-out/bin/self_print # macOS
LD_LIBRARY_PATH=../../target/release ./zig-out/bin/self_print   # Linux
```

Or use the build system run commands:
```sh
zig build run-basic
zig build run-self_print
```

### Static linking

```sh
# Build static library
cargo build --manifest-path ../../Cargo.toml --release

# Modify build.zig to use static linking, then:
zig build
./zig-out/bin/basic
./zig-out/bin/self_print
```

Notes: Uses Zig's `@cImport` to import `bat.h` and call C functions. The `build.zig` file configures include paths and library linking. For static builds, ensure `libbat_c.a` is available in `target/release`.

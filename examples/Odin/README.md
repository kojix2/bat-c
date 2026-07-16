## Odin examples

### Build and run

```sh
# From repo root
cargo build --release

cd examples/Odin
make
make run-basic
make run-self_print
```

Notes: Uses Odin's `foreign import` declarations to call C functions from `bat.h`. The Makefile links against the dynamic library in `target/release`.

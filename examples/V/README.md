## V examples

### Build and run

```sh
# From repo root
cargo build --release

cd examples/V
v run basic.v
v run self_print.v
```

Or compile first:
```sh
v basic.v
v self_print.v
DYLD_LIBRARY_PATH=../../target/release ./basic      # macOS
LD_LIBRARY_PATH=../../target/release ./basic        # Linux
DYLD_LIBRARY_PATH=../../target/release ./self_print # macOS
LD_LIBRARY_PATH=../../target/release ./self_print   # Linux
```

Notes: Uses V's `#flag` and `#include` directives to link with `libbat_c`. The examples embed rpath for easier execution. V's C interop allows direct declaration of C types and functions.

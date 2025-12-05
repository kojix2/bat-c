## C examples

### Build and run (recommended)

```sh
cd examples/C
make            # builds the Rust lib and both C binaries
make run-basic
make run-self_print
```

Notes: the C code only needs `#include "bat.h"`. The Makefile links with an rpath to `target/release`, and the `run-*` targets also set `LD_LIBRARY_PATH`/`DYLD_LIBRARY_PATH` for portability.

### Manual build (from repo root)

```sh
cargo build --release
cc examples/C/basic.c -I. -Ltarget/release -lbat_c -Wl,-rpath,target/release -o basic
cc examples/C/self_print.c -I. -Ltarget/release -lbat_c -Wl,-rpath,target/release -o self_print
LD_LIBRARY_PATH=target/release DYLD_LIBRARY_PATH=target/release ./basic      # Linux/macOS
LD_LIBRARY_PATH=target/release DYLD_LIBRARY_PATH=target/release ./self_print # Linux/macOS
```
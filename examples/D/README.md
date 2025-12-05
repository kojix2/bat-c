## D examples

### Build and run (dynamic linking)

```sh
# From repo root
cargo build --release

cd examples/D
make
make run-basic
make run-self_print
```

### Manual build

```sh
cargo build --release
cd examples/D

# macOS
dmd basic.d -of=basic -L-L../../target/release -L-lbat_c -L-rpath -L../../target/release
dmd self_print.d -of=self_print -L-L../../target/release -L-lbat_c -L-rpath -L../../target/release

# Linux
dmd basic.d -of=basic -L-L../../target/release -L-lbat_c -L-rpath=../../target/release
dmd self_print.d -of=self_print -L-L../../target/release -L-lbat_c -L-rpath=../../target/release

# Run
DYLD_LIBRARY_PATH=../../target/release ./basic      # macOS
LD_LIBRARY_PATH=../../target/release ./basic        # Linux
DYLD_LIBRARY_PATH=../../target/release ./self_print # macOS
LD_LIBRARY_PATH=../../target/release ./self_print   # Linux
```

### Using LDC (alternative D compiler)

```sh
# Replace dmd with ldc2 in the commands above
ldc2 basic.d -of=basic -L-L../../target/release -L-lbat_c -L-rpath=../../target/release
ldc2 self_print.d -of=self_print -L-L../../target/release -L-lbat_c -L-rpath=../../target/release
```

Notes: Uses D's `extern(C)` to call C functions from `libbat_c`. The Makefile embeds rpath for easier execution. D's interop with C is seamless - simply declare C functions with `extern(C)` and matching signatures.

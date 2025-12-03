## examples

```sh
cargo build --release
```

basic.c

```sh
cc examples/basic.c -I. -Ltarget/release -lbat_c -o basic
LD_LIBRARY_PATH=target/release ./basic # Linux
DYLD_LIBRARY_PATH=target/release ./basic # macOS
```

self_print.c

```sh
cc examples/self_print.c -I. -Ltarget/release -lbat_c -o self_print
LD_LIBRARY_PATH=target/release ./self_print # Linux
DYLD_LIBRARY_PATH=target/release ./self_print # macOS
```
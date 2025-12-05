## Ruby examples

### Build and run

```sh
# From repo root
cargo build --release

# Install FFI gem if needed
gem install ffi

# Run examples
ruby examples/Ruby/basic.rb
ruby examples/Ruby/self_print.rb
```

Notes: Uses Ruby's `ffi` gem to load `libbat_c.(dylib|so|dll)` from `target/release`. The examples mirror the C versions using `bat_pretty_print_to_string()` and `bat_free_string()`.

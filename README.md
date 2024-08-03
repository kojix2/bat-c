# bat-c

[![test](https://github.com/kojix2/bat-c/actions/workflows/test.yml/badge.svg)](https://github.com/kojix2/bat-c/actions/workflows/test.yml)

- Provides a C API for [bat](https://github.com/sharkdp/bat), enabling its use from various programming languages.

## Installation

Download binaries from [GitHub Releases](https://github.com/kojix2/bat-c/releases).

From source code:

```sh
git clone https://github.com/kojix2/bat-c
cd bat-c
cargo build --release
# target/release/libbat_c.so (Linux), libbat_c.dylib (macOS), bat_c.dll (Windows)
```

## API

Refer to the [bat documentation](https://docs.rs/bat).

### Example Usage in C

```c
#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * BatInputType enum to specify the type of input
 */
typedef enum BatInputType {
  BatBytes,
  BatFile,
  BatFiles,
} BatInputType;

/**
 * Struct to hold print options
 */
typedef struct BatPrintOptions {
  size_t tab_width;
  bool colored_output;
  bool true_color;
  bool header;
  bool line_numbers;
  bool grid;
  bool rule;
  bool show_nonprintable;
  bool snip;
  size_t wrapping_mode;
  bool use_italics;
  size_t paging_mode;
  size_t highlight_line;
} BatPrintOptions;

/**
 * Unified function to pretty print with specified options.
 */
void bat_print_pretty(const char *input,
                      size_t length,
                      enum BatInputType input_type,
                      const char *language,
                      const char *theme,
                      struct BatPrintOptions options);

```

## Development

### Running Tests

To run tests written in Rust:

```sh
cargo test
```

To run tests in C:

```sh
cd test && ./test.sh
```

### Creating Header Files

```sh
cargo install --force cbindgen
cbindgen --config cbindgen.toml --crate bat-c --output bat.h
```

## License

MIT

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
void print_pretty(const uint8_t *input,
                  size_t length,
                  const char *language,
                  size_t tab_width,
                  bool colored_output,
                  bool true_color,
                  bool header,
                  bool line_numbers,
                  bool grid,
                  bool rule,
                  bool show_nonprintable,
                  bool snip,
                  size_t wrapping_mode,
                  bool use_italics,
                  size_t paging_mode,
                  const char *theme,
                  size_t highlight_line);
```

```c
# include "bat.h"

int main() {
    const char* text = "<span style=\"color: #ff00cc\">Hello world!</span>\n";
    const char* language = "html";
    const char* theme = "TwoDark";

    print_pretty(
        (const unsigned char*)text,
        strlen(text),
        language,
        4,
        1,  // Enable colored_output
        1,  // Enable true_color
        1,  // Enable header
        1,  // Enable line_numbers
        1,  // Enable grid
        1,  // Enable rule
        1,  // Enable show_nonprintable
        1,  // Enable snip
        1,  // Set wrapping_mode to Character
        1,  // Enable use_italics
        1,  // Set paging_mode to Always
        theme,
        0   // Highlight the first line
    );
    return 0;
}
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

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
#include <stdio.h>
#include <string.h>

void print_pretty(
    const unsigned char* input,
    size_t length,
    const char* language,
    size_t tab_width,
    int colored_output,
    int true_color,
    int header,
    int line_numbers,
    int grid,
    int rule,
    int show_nonprintable,
    int snip,
    size_t wrapping_mode,
    int use_italics,
    size_t paging_mode,
    const char* theme,
    size_t highlight_line
);

int main() {
    const char* text = "<span style=\"color: #ff00cc\">Hello world!</span>\n";
    const char* language = "html";

    print_pretty(
        (const unsigned char*)text,
        strlen(text),
        language,
        4,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        0, // Never use pager
        NULL,
        0
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

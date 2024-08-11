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

### C API

```c
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
 * Pretty print with specified options.
 */
void bat_pretty_print(const char *input,
                      size_t length,
                      enum BatInputType input_type,
                      const char *language,
                      const char *theme,
                      struct BatPrintOptions options);

/**
 * Pretty print output to a string.
 */
const char *bat_pretty_print_to_string(const char *input,
                                       size_t length,
                                       enum BatInputType input_type,
                                       const char *language,
                                       const char *theme,
                                       struct BatPrintOptions options,
                                       size_t *output_length);

/**
 * Free the string allocated by `bat_pretty_print_to_string`.
 */
void bat_free_string(const char *s);

/**
 * Return the version of the library
 */
const char *bat_c_version(void);
```

Example:

```c
#include <stdio.h>
#include <string.h>
#include "../bat.h"

int main()
{
    const char *text = "<span style=\"color: #ff00cc\">Hello world!</span>\n";
    const char *language = "html";
    const char *theme = "Nord";

    BatPrintOptions options = {
        .tab_width = 4,
        .colored_output = 1,
        .true_color = 1,
        .header = 1,
        .line_numbers = 1,
        .grid = 1,
        .rule = 1,
        .show_nonprintable = 0,
        .snip = 1,
        .wrapping_mode = 1,
        .use_italics = 1,
        .paging_mode = 0,
        .highlight_line = 0};

    // Test with bytes input
    bat_pretty_print(
        text,
        strlen(text),
        BatBytes,
        language,
        theme,
        options);

    // Test with file input
    const char *file_path = "test_input.html";
    bat_pretty_print(
        file_path,
        0,
        BatFile,
        language,
        theme,
        options);

    // Test with multiple files input
    const char *file_paths[] = {"test_input1.html", "test_input2.html"};
    bat_pretty_print(
        (const char *)file_paths,
        2,
        BatFiles,
        language,
        theme,
        options);

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

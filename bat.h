/* https://github.com/kojix2/bat-c */

/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */

#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Pretty print the input string with specified options.
 * # Safety
 * This function is marked as unsafe because it dereferences raw pointers.
 */
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

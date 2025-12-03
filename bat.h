/* https://github.com/kojix2/bat-c */

#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * BatInputType enum to specify the type of input
 *
 * BatBytes: input is a pointer to a byte array, length is the number of bytes
 * BatFile:  input is a null-terminated file path string, length is ignored (can be 0)
 * BatFiles: input is a pointer to an array of const char* (file paths), length is the number of files
 */
typedef enum BatInputType {
  BatBytes,
  BatFile,
  BatFiles,
} BatInputType;

/**
 * Struct to hold print options
 *
 * tab_width: Number of spaces per tab (default: 4)
 * colored_output: Enable colored output
 * true_color: Use 24-bit colors
 * header: Show file header
 * line_numbers: Show line numbers
 * grid: Show grid borders
 * rule: Show horizontal rules between files
 * show_nonprintable: Show non-printable characters
 * snip: Show "snip" for long files
 * wrapping_mode: Text wrapping mode (0 = NoWrapping, 1 = Character)
 * use_italics: Use italic text
 * paging_mode: Paging mode (0 = Never, 1 = Always, 2 = QuitIfOneScreen)
 * highlight_line: Line number to highlight (0 = no highlight)
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
 * Print with specified options.
 *
 * Parameters:
 *   input: Depends on input_type (see BatInputType documentation)
 *   length: Depends on input_type (see BatInputType documentation)
 *   input_type: Type of input (BatBytes, BatFile, or BatFiles)
 *   language: Syntax highlighting language (can be NULL for auto-detection)
 *   theme: Color theme name (can be NULL for default theme)
 *   options: Print options
 *
 * Safety:
 *   - For BatBytes: caller must ensure input points to valid memory of at least length bytes
 *   - For BatFile: caller must ensure input is a valid null-terminated string
 *   - For BatFiles: caller must ensure input points to an array of length valid const char* pointers,
 *     each pointing to a valid null-terminated string
 *   - language and theme must be valid UTF-8 if not NULL
 *
 * Returns: 0 on success, 1 on error. Error details are printed to stderr.
 */
int32_t bat_pretty_print(const char *input,
                         size_t length,
                         enum BatInputType input_type,
                         const char *language,
                         const char *theme,
                         struct BatPrintOptions options);

/**
 * Pretty print output to a string.
 *
 * Parameters:
 *   input: Depends on input_type (see BatInputType documentation)
 *   length: Depends on input_type (see BatInputType documentation)
 *   input_type: Type of input (BatBytes, BatFile, or BatFiles)
 *   language: Syntax highlighting language (can be NULL for auto-detection)
 *   theme: Color theme name (can be NULL for default theme)
 *   options: Print options
 *   output: Pointer to receive the output string (allocated by this function)
 *   output_length: Pointer to receive the length of the output string in bytes
 *
 * Safety:
 *   - Same safety requirements as bat_pretty_print for input parameters
 *   - The returned string in *output is allocated by the library and MUST be freed
 *     by calling bat_free_string() exactly once
 *   - Do not free the returned string with free() or other memory management functions
 *   - output and output_length must be valid pointers
 *
 * Returns: 0 on success, 1 on error. Error details are printed to stderr.
 *          On error, *output will not be modified.
 */
int32_t bat_pretty_print_to_string(const char *input,
                                   size_t length,
                                   enum BatInputType input_type,
                                   const char *language,
                                   const char *theme,
                                   struct BatPrintOptions options,
                                   const char **output,
                                   size_t *output_length);

/**
 * Free the string allocated by bat_pretty_print_to_string.
 *
 * Parameters:
 *   s: Pointer returned by bat_pretty_print_to_string, or NULL
 *
 * Safety:
 *   - Only call this function on pointers returned by bat_pretty_print_to_string
 *   - Do not call this function twice on the same pointer (double-free)
 *   - Passing NULL is safe and does nothing
 *   - Do not pass pointers from malloc, stack variables, or other sources
 */
void bat_free_string(const char *s);

/**
 * Return the version of the library
 */
const char *bat_c_version(void);

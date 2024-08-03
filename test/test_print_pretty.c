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
    bat_print_pretty(
        text,
        strlen(text),
        BatBytes,
        language,
        theme,
        options);

    // Test with file input
    const char *file_path = "test_input.html";
    bat_print_pretty(
        file_path,
        0,
        BatFile,
        language,
        theme,
        options);

    // Test with multiple files input
    const char *file_paths[] = {"test_input1.html", "test_input2.html"};
    bat_print_pretty(
        (const char *)file_paths,
        2,
        BatFiles,
        language,
        theme,
        options);

    return 0;
}

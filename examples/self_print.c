#include <stdio.h>
#include "bat.h"

int main(void)
{
    // This example prints its own source code with bat-style syntax highlighting.
    const char *file = __FILE__;

    BatPrintOptions opt = {
        .tab_width = 4,
        .colored_output = 1,
        .true_color = 1,
        .header = 0,
        .line_numbers = 1,
        .grid = 0,
        .rule = 0,
        .show_nonprintable = 0,
        .snip = 1,
        .wrapping_mode = 1, // Character wrapping
        .use_italics = 1,
        .paging_mode = 0, // Never
        .highlight_line = 0,
    };

    const char *out = NULL;
    size_t len = 0;

    if (bat_pretty_print_to_string(
            file,
            0,       // length is ignored for BatFile
            BatFile, // treat input as a file path
            "c",     // language (NULL for auto-detection)
            "Nord",  // theme
            opt,
            &out,
            &len) != 0)
    {
        fprintf(stderr, "bat_pretty_print_to_string failed\n");
        return 1;
    }

    fwrite(out, 1, len, stdout);
    bat_free_string(out);
    return 0;
}
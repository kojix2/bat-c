#include <stdio.h>
#include "bat.h"

int main(void)
{
    // Highlight this source file.
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
        .wrapping_mode = 1, // char
        .use_italics = 1,
        .paging_mode = 0, // Never
        .highlight_line = 0,
    };

    const char *out = NULL;
    size_t len = 0;

    if (bat_pretty_print_to_string(
            file,
            0,       // len (unused)
            BatFile, // file path
            "c",     // language
            "Nord",  // theme
            opt,
            &out,
            &len) != 0)
    {
        fprintf(stderr, "error\n");
        return 1;
    }

    fwrite(out, 1, len, stdout);
    bat_free_string(out);
    return 0;
}
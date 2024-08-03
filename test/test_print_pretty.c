#include <stdio.h>
#include <string.h>
#include "../bat.h"

int main() {
    const char* text = "<span style=\"color: #ff00cc\">Hello world!</span>\n";
    const char* language = "html";
    const char* theme = "TwoDark";

    print_pretty(
        (const unsigned char*)text,
        strlen(text),
        language,
        4,  // Set tab_width to 4
        1,  // Enable colored_output
        1,  // Enable true_color
        1,  // Enable header
        1,  // Enable line_numbers
        1,  // Enable grid
        1,  // Enable rule
        0,  // Enable show_nonprintable
        1,  // Enable snip
        1,  // Set wrapping_mode to Character
        1,  // Enable use_italics
        1,  // Set paging_mode to Always
        theme,
        0   // Highlight the first line
    );
    return 0;
}
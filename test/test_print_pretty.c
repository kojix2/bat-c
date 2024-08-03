#include <stdio.h>
#include <string.h>
#include "../bat.h"

int main() {
    const char* text = "<span style=\"color: #ff00cc\">Hello world!</span>\n";
    const char* language = "html";

    // Call the print_pretty function
    print_pretty(
        (const unsigned char*)text,
        strlen(text),
        language,
        4,
        1,  // Enable colored_output
        0,  // Disable true_color
        0,  // Disable header
        0,  // Disable line_numbers
        0,  // Disable grid
        0,  // Disable rule
        0,  // Disable show_nonprintable
        0,  // Disable snip
        0,  // Set wrapping_mode to NoWrapping
        0,  // Disable use_italics
        0,  // Set paging_mode to Never
        NULL,
        0
    );

    return 0;
}

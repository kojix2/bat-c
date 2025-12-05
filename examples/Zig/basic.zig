const std = @import("std");
const c = @cImport({
    @cInclude("bat.h");
    @cInclude("stdio.h");
    @cInclude("string.h");
});

pub fn main() void {
    const text = "<span>Hello</span>\n";

    const opt = c.BatPrintOptions{
        .tab_width = 4,
        .colored_output = true,
        .true_color = true,
        .header = false,
        .line_numbers = false,
        .grid = false,
        .rule = false,
        .show_nonprintable = false,
        .snip = true,
        .wrapping_mode = 1,
        .use_italics = true,
        .paging_mode = 0,
        .highlight_line = 0,
    };

    var out: [*c]const u8 = null;
    var out_len: usize = 0;

    const ret = c.bat_pretty_print_to_string(
        text.ptr,
        text.len,
        0, // BatBytes
        "html",
        "Nord",
        opt,
        @ptrCast(&out),
        @ptrCast(&out_len),
    );

    if (ret != 0) {
        _ = c.fprintf(c.stderr(), "error\n");
        c.exit(1);
    }

    _ = c.fwrite(out, 1, out_len, c.stdout());
    c.bat_free_string(out);
}

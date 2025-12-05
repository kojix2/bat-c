const std = @import("std");
const c = @cImport({
    @cInclude("bat.h");
});

pub fn main() !void {
    const text = "<span>Hello</span>\n";

    const opt = c.BatPrintOptions{
        .tab_width = 4,
        .colored_output = 1,
        .true_color = 1,
        .header = 0,
        .line_numbers = 0,
        .grid = 0,
        .rule = 0,
        .show_nonprintable = 0,
        .snip = 1,
        .wrapping_mode = 1,
        .use_italics = 1,
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
        const stderr = std.io.getStdErr().writer();
        try stderr.writeAll("error\n");
        std.process.exit(1);
    }

    const stdout = std.io.getStdOut().writer();
    try stdout.writeAll(out[0..out_len]);
    c.bat_free_string(out);
}

const std = @import("std");
const bat = @import("bat_c.zig");

pub fn main() !void {
    const text = "<span>Hello</span>\n";

    const opt = bat.BatPrintOptions{
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

    const ret = bat.bat_pretty_print_to_string(
        text.ptr,
        text.len,
        bat.BatBytes,
        "html",
        "Nord",
        opt,
        &out,
        &out_len,
    );

    if (ret != 0) {
        std.debug.print("error\n", .{});
        std.process.exit(1);
    }

    try bat.writeStdout(out[0..out_len]);
    bat.bat_free_string(out);
}

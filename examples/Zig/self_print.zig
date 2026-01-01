const std = @import("std");
const c = @cImport({
    @cInclude("bat.h");
});

pub fn main() void {
    const file_path = "self_print.zig";

    const opt = c.BatPrintOptions{
        .tab_width = 4,
        .colored_output = true,
        .true_color = true,
        .header = false,
        .line_numbers = true,
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
        file_path.ptr,
        0,
        1, // BatFile
        "zig",
        "Nord",
        opt,
        @ptrCast(&out),
        @ptrCast(&out_len),
    );

    if (ret != 0) {
        std.debug.print("error\n", .{});
        std.process.exit(1);
    }

    _ = std.c.write(1, out, out_len);
    c.bat_free_string(out);
}

pub const BatPrintOptions = extern struct {
    tab_width: usize,
    colored_output: bool,
    true_color: bool,
    header: bool,
    line_numbers: bool,
    grid: bool,
    rule: bool,
    show_nonprintable: bool,
    snip: bool,
    wrapping_mode: usize,
    use_italics: bool,
    paging_mode: usize,
    highlight_line: usize,
};

pub const BatBytes: i32 = 0;
pub const BatFile: i32 = 1;

extern "c" fn write(fd: i32, buf: [*]const u8, count: usize) isize;

pub fn writeStdout(bytes: []const u8) !void {
    var written: usize = 0;
    while (written < bytes.len) {
        const chunk = bytes[written..];
        const n = write(1, chunk.ptr, chunk.len);
        if (n <= 0) return error.WriteFailed;
        written += @intCast(n);
    }
}

pub extern "c" fn bat_pretty_print_to_string(
    input: [*c]const u8,
    length: usize,
    input_type: i32,
    language: [*c]const u8,
    theme: [*c]const u8,
    options: BatPrintOptions,
    output: *[*c]const u8,
    output_length: *usize,
) i32;

pub extern "c" fn bat_free_string(s: [*c]const u8) void;

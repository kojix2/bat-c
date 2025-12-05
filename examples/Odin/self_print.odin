package main

import "core:c"
import "core:fmt"
import "core:os"
import "core:runtime"
import "core:strings"

foreign import bat "../../target/release/libbat_c.dylib"

BatPrintOptions :: struct {
	tab_width:          u32,
	colored_output:     u8,
	true_color:         u8,
	header:             u8,
	line_numbers:       u8,
	grid:               u8,
	rule:               u8,
	show_nonprintable:  u8,
	snip:               u8,
	wrapping_mode:      c.size_t,
	use_italics:        u8,
	paging_mode:        c.size_t,
	highlight_line:     c.size_t,
}

foreign bat {
	bat_pretty_print_to_string :: proc(
input: rawptr,
input_len: c.size_t,
input_type: c.int,
language: cstring,
theme: cstring,
options: BatPrintOptions,
output: ^cstring,
output_len: ^c.size_t,
) -> c.int ---
	
	bat_free_string :: proc(s: cstring) ---
}

main :: proc() {
	file_path := "self_print.odin"
	opt := BatPrintOptions{
		tab_width = 4,
		colored_output = 1,
		true_color = 1,
		header = 0,
		line_numbers = 1,
		grid = 0,
		rule = 0,
		show_nonprintable = 0,
		snip = 1,
		wrapping_mode = 1,
		use_italics = 1,
		paging_mode = 0,
		highlight_line = 0,
	}
	
	output: cstring
	output_len: c.size_t
	
	ret := bat_pretty_print_to_string(
raw_data(file_path),
0,
1,
"odin",
"Nord",
opt,
&output,
		&output_len,
	)
	
	if ret != 0 {
		fmt.eprintln("error")
		os.exit(1)
	}
	
	result := transmute([]byte)runtime.Raw_String{data = cast(^byte)output, len = int(output_len)}
	os.write(os.stdout, result)
	bat_free_string(output)
}

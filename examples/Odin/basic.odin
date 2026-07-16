package main

import "core:c"
import "core:fmt"
import "core:os"
import "core:strings"

foreign import lib "system:bat_c"

BatPrintOptions :: struct {
	tab_width:         c.size_t,
	colored_output:    b8,
	true_color:        b8,
	header:            b8,
	line_numbers:      b8,
	grid:              b8,
	rule:              b8,
	show_nonprintable: b8,
	snip:              b8,
	wrapping_mode:     c.size_t,
	use_italics:       b8,
	paging_mode:       c.size_t,
	highlight_line:    c.size_t,
}

foreign lib {
	bat_pretty_print_to_string :: proc(
		input: rawptr,
		length: c.size_t,
		input_type: i32,
		language: cstring,
		theme: cstring,
		options: BatPrintOptions,
		output: ^cstring,
		output_length: ^c.size_t,
	) -> i32 ---

	bat_free_string :: proc(s: cstring) ---
}

main :: proc() {
	text := "<span>Hello</span>\n"
	opt := BatPrintOptions {
		tab_width = 4,
		colored_output = true,
		true_color = true,
		header = false,
		line_numbers = false,
		grid = false,
		rule = false,
		show_nonprintable = false,
		snip = true,
		wrapping_mode = 1,
		use_italics = true,
		paging_mode = 0,
		highlight_line = 0,
	}

	output: cstring
	output_len: c.size_t

	ret := bat_pretty_print_to_string(
		raw_data(text),
		c.size_t(len(text)),
		0,
		"html",
		"Nord",
		opt,
		&output,
		&output_len,
	)
	if ret != 0 {
		fmt.eprintln("error")
		os.exit(1)
	}
	defer bat_free_string(output)

	fmt.print(strings.string_from_ptr((^u8)(output), int(output_len)))
}

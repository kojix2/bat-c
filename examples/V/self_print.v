module main

import os

#flag -I @VMODROOT/../..
#flag -L @VMODROOT/../../target/release
#flag -lbat_c
#flag darwin -Wl,-rpath,@VMODROOT/../../target/release
#flag linux -Wl,-rpath=@VMODROOT/../../target/release
#include "bat.h"

struct C.BatPrintOptions {
	tab_width          u32
	colored_output     u8
	true_color         u8
	header             u8
	line_numbers       u8
	grid               u8
	rule               u8
	show_nonprintable  u8
	snip               u8
	wrapping_mode      usize
	use_italics        u8
	paging_mode        usize
	highlight_line     usize
}

fn C.bat_pretty_print_to_string(input &u8, input_len usize, input_type int, language &char, theme &char, options C.BatPrintOptions, output &&char, output_len &usize) int

fn C.bat_free_string(s &char)

fn main() {
	file_path := 'self_print.v'
	opt := C.BatPrintOptions{
		tab_width: 4
		colored_output: 1
		true_color: 1
		header: 0
		line_numbers: 1
		grid: 0
		rule: 0
		show_nonprintable: 0
		snip: 1
		wrapping_mode: 1
		use_italics: 1
		paging_mode: 0
		highlight_line: 0
	}

	mut output := &char(0)
	mut output_len := usize(0)

	ret := C.bat_pretty_print_to_string(file_path.str, 0, 1, c'v', c'Nord', opt,
&output, &output_len)

	if ret != 0 {
		eprintln('error')
		exit(1)
	}

	unsafe {
		mut bytes := []u8{len: int(output_len)}
		C.memcpy(bytes.data, output, output_len)
		print(bytes.bytestr())
	}
	C.bat_free_string(output)
}

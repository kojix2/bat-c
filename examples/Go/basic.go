package main

/*
#cgo CFLAGS: -I../..
#cgo LDFLAGS: -L../../target/release -lbat_c
#include "bat.h"
#include <string.h>
*/
import "C"
import (
	"fmt"
	"os"
	"unsafe"
)

func main() {
	text := "<span>Hello</span>\n"

	opt := C.BatPrintOptions{
		tab_width:         4,
		colored_output:    C._Bool(true),
		true_color:        C._Bool(true),
		header:            C._Bool(false),
		line_numbers:      C._Bool(false),
		grid:              C._Bool(false),
		rule:              C._Bool(false),
		show_nonprintable: C._Bool(false),
		snip:              C._Bool(true),
		wrapping_mode:     1,
		use_italics:       C._Bool(true),
		paging_mode:       0,
		highlight_line:    0,
	}

	var out *C.char
	var outLen C.size_t

	ret := C.bat_pretty_print_to_string(
		C.CString(text),
		C.size_t(len(text)),
		0, // BatBytes
		C.CString("html"),
		C.CString("Nord"),
		opt,
		&out,
		&outLen,
	)

	if ret != 0 {
		fmt.Fprintln(os.Stderr, "error")
		os.Exit(1)
	}

	result := C.GoBytes(unsafe.Pointer(out), C.int(outLen))
	os.Stdout.Write(result)
	C.bat_free_string(out)
}

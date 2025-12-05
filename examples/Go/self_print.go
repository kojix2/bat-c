package main

/*
#cgo CFLAGS: -I../..
#cgo LDFLAGS: -L../../target/release -lbat_c
#include "bat.h"
#include <stdlib.h>
*/
import "C"
import (
	"fmt"
	"os"
	"unsafe"
)

func main() {
	filePath := C.CString("self_print.go")
	defer C.free(unsafe.Pointer(filePath))

	opt := C.BatPrintOptions{
		tab_width:         4,
		colored_output:    1,
		true_color:        1,
		header:            0,
		line_numbers:      1,
		grid:              0,
		rule:              0,
		show_nonprintable: 0,
		snip:              1,
		wrapping_mode:     1,
		use_italics:       1,
		paging_mode:       0,
		highlight_line:    0,
	}

	var out *C.char
	var outLen C.size_t

	lang := C.CString("go")
	defer C.free(unsafe.Pointer(lang))
	theme := C.CString("Nord")
	defer C.free(unsafe.Pointer(theme))

	ret := C.bat_pretty_print_to_string(
		unsafe.Pointer(filePath),
		0,
		C.int(1), // BatFile
		lang,
		theme,
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

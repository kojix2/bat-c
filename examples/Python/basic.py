#!/usr/bin/env python3
import ctypes
import sys
from pathlib import Path

# Load the shared library
lib_path = Path(__file__).parent.parent.parent / "target" / "release"
if sys.platform == "darwin":
    lib = ctypes.CDLL(str(lib_path / "libbat_c.dylib"))
elif sys.platform == "win32":
    lib = ctypes.CDLL(str(lib_path / "bat_c.dll"))
else:
    lib = ctypes.CDLL(str(lib_path / "libbat_c.so"))

# Define BatPrintOptions struct
class BatPrintOptions(ctypes.Structure):
    _fields_ = [
        ("tab_width", ctypes.c_uint32),
        ("colored_output", ctypes.c_uint8),
        ("true_color", ctypes.c_uint8),
        ("header", ctypes.c_uint8),
        ("line_numbers", ctypes.c_uint8),
        ("grid", ctypes.c_uint8),
        ("rule", ctypes.c_uint8),
        ("show_nonprintable", ctypes.c_uint8),
        ("snip", ctypes.c_uint8),
        ("wrapping_mode", ctypes.c_size_t),
        ("use_italics", ctypes.c_uint8),
        ("paging_mode", ctypes.c_size_t),
        ("highlight_line", ctypes.c_size_t),
    ]

# Function signatures
lib.bat_pretty_print_to_string.argtypes = [
    ctypes.c_void_p, ctypes.c_size_t, ctypes.c_int,
    ctypes.c_char_p, ctypes.c_char_p, BatPrintOptions,
    ctypes.POINTER(ctypes.c_char_p), ctypes.POINTER(ctypes.c_size_t),
]
lib.bat_pretty_print_to_string.restype = ctypes.c_int
lib.bat_free_string.argtypes = [ctypes.c_char_p]
lib.bat_free_string.restype = None

def main():
    text = b"<span>Hello</span>\n"
    opt = BatPrintOptions(
        tab_width=4, colored_output=1, true_color=1, header=0,
        line_numbers=0, grid=0, rule=0, show_nonprintable=0,
        snip=1, wrapping_mode=1, use_italics=1, paging_mode=0, highlight_line=0,
    )
    out = ctypes.c_char_p()
    out_len = ctypes.c_size_t()
    
    ret = lib.bat_pretty_print_to_string(
        text, len(text), 0, b"html", b"Nord", opt,
        ctypes.byref(out), ctypes.byref(out_len)
    )
    if ret != 0:
        print("error", file=sys.stderr)
        return 1
    
    sys.stdout.buffer.write(out.value[:out_len.value])
    lib.bat_free_string(out)
    return 0

if __name__ == "__main__":
    sys.exit(main())

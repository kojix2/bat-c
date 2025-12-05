#!/usr/bin/env julia

# Load the shared library
const lib_path = joinpath(@__DIR__, "..", "..", "target", "release")
const libbat = if Sys.isapple()
    joinpath(lib_path, "libbat_c.dylib")
elseif Sys.iswindows()
    joinpath(lib_path, "bat_c.dll")
else
    joinpath(lib_path, "libbat_c.so")
end

# BatPrintOptions struct
struct BatPrintOptions
    tab_width::UInt32
    colored_output::UInt8
    true_color::UInt8
    header::UInt8
    line_numbers::UInt8
    grid::UInt8
    rule::UInt8
    show_nonprintable::UInt8
    snip::UInt8
    wrapping_mode::Csize_t
    use_italics::UInt8
    paging_mode::Csize_t
    highlight_line::Csize_t
end

function main()
    text = "<span>Hello</span>\n"
    opt = BatPrintOptions(
        4, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0
    )
    
    out = Ref{Ptr{UInt8}}(C_NULL)
    out_len = Ref{Csize_t}(0)
    
    ret = ccall(
        (:bat_pretty_print_to_string, libbat), Cint,
        (Ptr{UInt8}, Csize_t, Cint, Cstring, Cstring, BatPrintOptions, Ptr{Ptr{UInt8}}, Ptr{Csize_t}),
        text, sizeof(text), 0, "html", "Nord", opt, out, out_len
    )
    
    if ret != 0
        println(stderr, "error")
        exit(1)
    end
    
    result = unsafe_wrap(Array, out[], out_len[])
    write(stdout, result)
    
    ccall((:bat_free_string, libbat), Cvoid, (Ptr{UInt8},), out[])
end

main()

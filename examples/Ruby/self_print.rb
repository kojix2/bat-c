#!/usr/bin/env ruby
require 'ffi'

module BatC
  extend FFI::Library
  
  lib_path = File.expand_path('../../target/release', __dir__)
  case RbConfig::CONFIG['host_os']
  when /darwin/
    ffi_lib File.join(lib_path, 'libbat_c.dylib')
  when /linux/
    ffi_lib File.join(lib_path, 'libbat_c.so')
  when /mswin|mingw|cygwin/
    ffi_lib File.join(lib_path, 'bat_c.dll')
  end

  class BatPrintOptions < FFI::Struct
    layout :tab_width, :uint32,
           :colored_output, :uint8,
           :true_color, :uint8,
           :header, :uint8,
           :line_numbers, :uint8,
           :grid, :uint8,
           :rule, :uint8,
           :show_nonprintable, :uint8,
           :snip, :uint8,
           :wrapping_mode, :size_t,
           :use_italics, :uint8,
           :paging_mode, :size_t,
           :highlight_line, :size_t
  end

  attach_function :bat_pretty_print_to_string, [
    :pointer, :size_t, :int,
    :string, :string, BatPrintOptions.by_value,
    :pointer, :pointer
  ], :int
  
  attach_function :bat_free_string, [:pointer], :void
end

file_path = __FILE__
opt = BatC::BatPrintOptions.new
opt[:tab_width] = 4
opt[:colored_output] = 1
opt[:true_color] = 1
opt[:header] = 0
opt[:line_numbers] = 1
opt[:grid] = 0
opt[:rule] = 0
opt[:show_nonprintable] = 0
opt[:snip] = 1
opt[:wrapping_mode] = 1
opt[:use_italics] = 1
opt[:paging_mode] = 0
opt[:highlight_line] = 0

out_ptr = FFI::MemoryPointer.new(:pointer)
out_len = FFI::MemoryPointer.new(:size_t)

ret = BatC.bat_pretty_print_to_string(
  FFI::MemoryPointer.from_string(file_path), 0, 1,
  "ruby", "Nord", opt, out_ptr, out_len
)

if ret != 0
  $stderr.puts "error"
  exit 1
end

output = out_ptr.read_pointer.read_string(out_len.read(:size_t))
$stdout.write(output)
BatC.bat_free_string(out_ptr.read_pointer)

import os

const rootPath = currentSourcePath.parentDir.parentDir.parentDir
const libPath = rootPath / "target" / "release"

{.passL: "-L" & libPath & " -lbat_c".}
when defined(macosx):
  {.passL: "-Wl,-rpath," & libPath.}
else:
  {.passL: "-Wl,-rpath=" & libPath.}

type
  BatPrintOptions {.importc, header: rootPath / "bat.h".} = object
    tab_width: uint32
    colored_output: uint8
    true_color: uint8
    header: uint8
    line_numbers: uint8
    grid: uint8
    rule: uint8
    show_nonprintable: uint8
    snip: uint8
    wrapping_mode: csize_t
    use_italics: uint8
    paging_mode: csize_t
    highlight_line: csize_t

proc bat_pretty_print_to_string(
  input: pointer, input_len: csize_t, input_type: cint,
  language: cstring, theme: cstring, options: BatPrintOptions,
  output: ptr cstring, output_len: ptr csize_t
): cint {.importc, header: rootPath / "bat.h".}

proc bat_free_string(s: cstring) {.importc, header: rootPath / "bat.h".}

proc main() =
  let filePath = currentSourcePath
  var opt = BatPrintOptions(
    tab_width: 4, colored_output: 1, true_color: 1, header: 0,
    line_numbers: 1, grid: 0, rule: 0, show_nonprintable: 0,
    snip: 1, wrapping_mode: 1, use_italics: 1, paging_mode: 0, highlight_line: 0
  )

  var output: cstring
  var output_len: csize_t

  let ret = bat_pretty_print_to_string(
    cstring(filePath), 0, 1,
    "nim", "Nord", opt, addr output, addr output_len
  )

  if ret != 0:
    stderr.writeLine("error")
    quit(1)

  stdout.write($output)
  bat_free_string(output)

main()

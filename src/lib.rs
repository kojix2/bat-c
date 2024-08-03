extern crate bat;

use bat::{Input, PrettyPrinter};
use std::ffi::CStr;
use std::os::raw::c_char;
use std::slice;
use std::str;

/// Pretty print the input string with specified options.
/// # Safety
/// This function is marked as unsafe because it dereferences raw pointers.
#[no_mangle]
pub unsafe extern "C" fn print_pretty(
    input: *const u8,
    length: usize,
    language: *const c_char,
    tab_width: usize,
    colored_output: bool,
    true_color: bool,
    header: bool,
    line_numbers: bool,
    grid: bool,
    rule: bool,
    show_nonprintable: bool,
    snip: bool,
    wrapping_mode: usize, // WrappingMode is represented as usize for simplicity
    use_italics: bool,
    paging_mode: usize, // PagingMode is represented as usize for simplicity
    theme: *const c_char,
    highlight_line: usize,
) {
    // Validate and convert input slice
    if input.is_null() || length == 0 {
        return; // Early return on invalid input
    }
    let slice = slice::from_raw_parts(input, length);
    let input_str = match str::from_utf8(slice) {
        Ok(s) => s,
        Err(_) => return, // Early return on invalid UTF-8
    };

    // Validate and convert language string
    let language_str = if !language.is_null() {
        match CStr::from_ptr(language).to_str() {
            Ok(s) => s,
            Err(_) => "",
        }
    } else {
        ""
    };

    // Validate and convert theme string
    let theme_str = if !theme.is_null() {
        match CStr::from_ptr(theme).to_str() {
            Ok(s) => s,
            Err(_) => "",
        }
    } else {
        ""
    };

    // Map wrapping_mode usize to WrappingMode enum
    let wrapping_mode_enum = match wrapping_mode {
        1 => bat::WrappingMode::Character,
        2 => bat::WrappingMode::NoWrapping(false), // Adjusted to match constructor
        _ => bat::WrappingMode::NoWrapping(false),
    };

    // Map paging_mode usize to PagingMode enum
    let paging_mode_enum = match paging_mode {
        1 => bat::PagingMode::Always,
        2 => bat::PagingMode::QuitIfOneScreen,
        _ => bat::PagingMode::Never,
    };

    // Create and configure PrettyPrinter instance
    PrettyPrinter::new()
        .input(Input::from_bytes(input_str.as_bytes()))
        .language(language_str)
        .tab_width(Some(tab_width))
        .colored_output(colored_output)
        .true_color(true_color)
        .header(header)
        .line_numbers(line_numbers)
        .grid(grid)
        .rule(rule)
        .show_nonprintable(show_nonprintable)
        .snip(snip)
        .wrapping_mode(wrapping_mode_enum)
        .use_italics(use_italics)
        .paging_mode(paging_mode_enum)
        .theme(theme_str)
        .highlight(highlight_line)
        .print()
        .unwrap_or_else(|e| {
            eprintln!("Error printing: {}", e);
            false // Return false to match expected type
        });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_print_pretty() {
        let input = "<span style=\"color: #ff00cc\">Hello world!</span>\n";
        let input_cstr = CString::new(input).unwrap();
        let language_cstr = CString::new("html").unwrap();

        unsafe {
            print_pretty(
                input_cstr.as_ptr() as *const u8,
                input.len(),
                language_cstr.as_ptr(),
                4,
                true,
                true,
                true,
                true,
                true,
                true,
                true,
                true,
                1,
                true,
                0,
                std::ptr::null(),
                0,
            );
        }
    }
}

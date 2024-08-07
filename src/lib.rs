extern crate bat;

use bat::{Input, PrettyPrinter};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::Path;
use std::slice;
use std::str;

/// BatInputType enum to specify the type of input
#[repr(C)]
pub enum BatInputType {
    BatBytes,
    BatFile,
    BatFiles,
}

/// Struct to hold print options
#[repr(C)]
pub struct BatPrintOptions {
    tab_width: usize,
    colored_output: bool,
    true_color: bool,
    header: bool,
    line_numbers: bool,
    grid: bool,
    rule: bool,
    show_nonprintable: bool,
    snip: bool,
    wrapping_mode: usize,
    use_italics: bool,
    paging_mode: usize,
    highlight_line: usize,
}

/// Unified function to pretty print with specified options.
/// # Safety
/// This function is marked as unsafe because it dereferences raw pointers.
#[no_mangle]
pub unsafe extern "C" fn bat_print_pretty(
    input: *const c_char,
    length: usize,
    input_type: BatInputType,
    language: *const c_char,
    theme: *const c_char,
    options: BatPrintOptions,
) {
    let language_str = if !language.is_null() {
        CStr::from_ptr(language).to_str().unwrap_or("")
    } else {
        ""
    };

    let theme_str = if !theme.is_null() {
        CStr::from_ptr(theme).to_str().unwrap_or("")
    } else {
        ""
    };

    let wrapping_mode_enum = match options.wrapping_mode {
        1 => bat::WrappingMode::Character,
        2 => bat::WrappingMode::NoWrapping(false),
        _ => bat::WrappingMode::NoWrapping(false),
    };

    let paging_mode_enum = match options.paging_mode {
        1 => bat::PagingMode::Always,
        2 => bat::PagingMode::QuitIfOneScreen,
        _ => bat::PagingMode::Never,
    };

    let mut printer_binding = PrettyPrinter::new();
    let printer = printer_binding
        .language(language_str)
        .tab_width(Some(options.tab_width))
        .colored_output(options.colored_output)
        .true_color(options.true_color)
        .header(options.header)
        .line_numbers(options.line_numbers)
        .grid(options.grid)
        .rule(options.rule)
        .show_nonprintable(options.show_nonprintable)
        .snip(options.snip)
        .wrapping_mode(wrapping_mode_enum)
        .use_italics(options.use_italics)
        .paging_mode(paging_mode_enum)
        .theme(theme_str)
        .highlight(options.highlight_line);

    match input_type {
        BatInputType::BatBytes => {
            if !input.is_null() && length > 0 {
                let slice = slice::from_raw_parts(input as *const u8, length);
                if let Ok(input_str) = str::from_utf8(slice) {
                    printer.input(Input::from_bytes(input_str.as_bytes()));
                }
            }
        }
        BatInputType::BatFile => {
            if !input.is_null() {
                if let Ok(file_path) = CStr::from_ptr(input).to_str() {
                    printer.input_file(Path::new(file_path));
                }
            }
        }
        BatInputType::BatFiles => {
            if !input.is_null() && length > 0 {
                let slice = slice::from_raw_parts(input as *const *const c_char, length);
                let paths: Vec<&Path> = slice
                    .iter()
                    .filter_map(|&ptr| {
                        if !ptr.is_null() {
                            CStr::from_ptr(ptr).to_str().ok().map(Path::new)
                        } else {
                            None
                        }
                    })
                    .collect();
                printer.input_files(paths);
            }
        }
    }

    printer.print().unwrap_or_else(|e| {
        eprintln!("Error printing: {}", e);
        false
    });
}

// Return the version of the library
#[no_mangle]
pub extern "C" fn bat_c_version() -> *const c_char {
    let version = env!("CARGO_PKG_VERSION");
    let version_cstr = CString::new(version).unwrap();
    version_cstr.into_raw()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_bat_print_pretty_bytes() {
        let input = "<span style=\"color: #ff00cc\">Hello world!</span>\n";
        let input_cstr = CString::new(input).unwrap();
        let language_cstr = CString::new("html").unwrap();
        let options = BatPrintOptions {
            tab_width: 4,
            colored_output: true,
            true_color: true,
            header: true,
            line_numbers: true,
            grid: true,
            rule: true,
            show_nonprintable: false,
            snip: true,
            wrapping_mode: 1,
            use_italics: true,
            paging_mode: 0,
            highlight_line: 0,
        };

        unsafe {
            bat_print_pretty(
                input_cstr.as_ptr(),
                input.len(),
                BatInputType::BatBytes,
                language_cstr.as_ptr(),
                std::ptr::null(),
                options,
            );
        }
    }

    #[test]
    fn test_bat_print_pretty_file() {
        let file_path = "test/test_input.html";
        let file_path_cstr = CString::new(file_path).unwrap();
        let language_cstr = CString::new("html").unwrap();
        let options = BatPrintOptions {
            tab_width: 4,
            colored_output: true,
            true_color: true,
            header: true,
            line_numbers: true,
            grid: true,
            rule: true,
            show_nonprintable: false,
            snip: true,
            wrapping_mode: 1,
            use_italics: true,
            paging_mode: 0,
            highlight_line: 0,
        };

        unsafe {
            bat_print_pretty(
                file_path_cstr.as_ptr(),
                0,
                BatInputType::BatFile,
                language_cstr.as_ptr(),
                std::ptr::null(),
                options,
            );
        }
    }

    #[test]
    fn test_bat_print_pretty_files() {
        let file_paths = ["test/test_input1.html", "test/test_input2.html"];
        let file_paths_cstr: Vec<CString> = file_paths
            .iter()
            .map(|&s| CString::new(s).unwrap())
            .collect();
        let file_paths_ptr: Vec<*const c_char> =
            file_paths_cstr.iter().map(|s| s.as_ptr()).collect();
        let language_cstr = CString::new("html").unwrap();
        let options = BatPrintOptions {
            tab_width: 4,
            colored_output: true,
            true_color: true,
            header: true,
            line_numbers: true,
            grid: true,
            rule: true,
            show_nonprintable: false,
            snip: true,
            wrapping_mode: 1,
            use_italics: true,
            paging_mode: 0,
            highlight_line: 0,
        };

        unsafe {
            bat_print_pretty(
                file_paths_ptr.as_ptr() as *const c_char,
                file_paths.len(),
                BatInputType::BatFiles,
                language_cstr.as_ptr(),
                std::ptr::null(),
                options,
            );
        }
    }

    #[test]
    fn test_bat_c_version() {
        let version = unsafe { CStr::from_ptr(bat_c_version()).to_str().unwrap() };
        assert_eq!(version, env!("CARGO_PKG_VERSION"));
    }
}

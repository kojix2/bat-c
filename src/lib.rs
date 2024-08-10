extern crate bat;

use bat::{Input, PrettyPrinter};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::Path;
use std::slice;

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

impl BatPrintOptions {
    fn apply_to_printer(&self, printer: &mut PrettyPrinter) {
        printer
            .tab_width(Some(self.tab_width))
            .colored_output(self.colored_output)
            .true_color(self.true_color)
            .header(self.header)
            .line_numbers(self.line_numbers)
            .grid(self.grid)
            .rule(self.rule)
            .show_nonprintable(self.show_nonprintable)
            .snip(self.snip)
            .wrapping_mode(convert_wrapping_mode(self.wrapping_mode))
            .use_italics(self.use_italics)
            .paging_mode(convert_paging_mode(self.paging_mode))
            .highlight(self.highlight_line);
    }
}

/// Convert a raw C string pointer to a Rust string slice.
fn to_str<'a>(cstr: *const c_char) -> Result<&'a str, std::str::Utf8Error> {
    unsafe { CStr::from_ptr(cstr).to_str() }
}

/// Convert `wrapping_mode` and `paging_mode` to respective enums.
fn convert_wrapping_mode(mode: usize) -> bat::WrappingMode {
    match mode {
        1 => bat::WrappingMode::Character,
        _ => bat::WrappingMode::NoWrapping(false),
    }
}

fn convert_paging_mode(mode: usize) -> bat::PagingMode {
    match mode {
        1 => bat::PagingMode::Always,
        2 => bat::PagingMode::QuitIfOneScreen,
        _ => bat::PagingMode::Never,
    }
}

/// Set the input for the printer based on input type.
fn set_input(
    printer: &mut PrettyPrinter,
    input_type: BatInputType,
    input: *const c_char,
    length: usize,
) -> Result<(), String> {
    match input_type {
        BatInputType::BatBytes => {
            let input_str = unsafe {
                std::str::from_utf8(slice::from_raw_parts(input as *const u8, length))
                    .map_err(|e| e.to_string())?
            };
            printer.input(Input::from_bytes(input_str.as_bytes()));
        }
        BatInputType::BatFile => {
            let file_path = to_str(input).map_err(|e| e.to_string())?;
            printer.input_file(Path::new(file_path));
        }
        BatInputType::BatFiles => {
            let paths: Vec<&Path> = unsafe {
                slice::from_raw_parts(input as *const *const c_char, length)
                    .iter()
                    .filter_map(|&ptr| CStr::from_ptr(ptr).to_str().ok().map(Path::new))
                    .collect()
            };
            printer.input_files(paths);
        }
    }
    Ok(())
}

/// Common logic for pretty printing
fn execute_pretty_print(
    input: *const c_char,
    length: usize,
    input_type: BatInputType,
    language: Option<&str>,
    theme: Option<&str>,
    options: BatPrintOptions,
) -> Result<(), String> {
    let mut printer = PrettyPrinter::new();

    if language.is_some() {
        printer.language(language.unwrap());
    }

    if theme.is_some() {
        printer.theme(theme.unwrap());
    }
    options.apply_to_printer(&mut printer);

    set_input(&mut printer, input_type, input, length)?;

    if printer.print().is_err() {
        Err("Error printing.".into())
    } else {
        Ok(())
    }
}

/// Pretty print with specified options.
/// # Safety
/// This function is marked as unsafe because it dereferences raw pointers.
#[no_mangle]
pub unsafe extern "C" fn bat_pretty_print(
    input: *const c_char,
    length: usize,
    input_type: BatInputType,
    language: *const c_char,
    theme: *const c_char,
    options: BatPrintOptions,
) {
    let language = if language.is_null() {
        None
    } else {
        Some(to_str(language).unwrap())
    };

    let theme = if theme.is_null() {
        None
    } else {
        Some(to_str(theme).unwrap())
    };

    if let Err(err) = execute_pretty_print(input, length, input_type, language, theme, options) {
        eprintln!("{}", err);
    }
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
            bat_pretty_print(
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
            bat_pretty_print(
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
            bat_pretty_print(
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

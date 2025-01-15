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

fn prepare_printer<'a>(
    input: *const c_char,
    length: usize,
    input_type: BatInputType,
    language: Option<&'a str>,
    theme: Option<&'a str>,
    options: BatPrintOptions,
) -> Result<PrettyPrinter<'a>, String> {
    let mut printer = PrettyPrinter::new();

    if let Some(lang) = language {
        printer.language(lang);
    }

    if let Some(thm) = theme {
        printer.theme(thm);
    }

    options.apply_to_printer(&mut printer);

    set_input(&mut printer, input_type, input, length)?;

    Ok(printer)
}

fn execute_pretty_print(
    input: *const c_char,
    length: usize,
    input_type: BatInputType,
    language: Option<&str>,
    theme: Option<&str>,
    options: BatPrintOptions,
) -> Result<(), String> {
    let mut printer = prepare_printer(input, length, input_type, language, theme, options)?;

    if printer.print().is_err() {
        Err("Error printing.".into())
    } else {
        Ok(())
    }
}

/// Print with specified options.
/// If input and length are invalid or mismatched,　undefined behavior may occur.
/// The caller must ensure valid memory and the correct length.
/// Returns 0 on success, 1 on error.
#[no_mangle]
pub unsafe extern "C" fn bat_pretty_print(
    input: *const c_char,
    length: usize,
    input_type: BatInputType,
    language: *const c_char,
    theme: *const c_char,
    options: BatPrintOptions,
) -> i32 {
    let language = if language.is_null() {
        None
    } else {
        match to_str(language) {
            Ok(lang) => Some(lang),
            Err(err) => {
                eprintln!("{}", err);
                return 1;
            }
        }
    };

    let theme = if theme.is_null() {
        None
    } else {
        Some(to_str(theme).unwrap())
    };

    match execute_pretty_print(input, length, input_type, language, theme, options) {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("{}", err);
            1
        }
    }
}

/// Pretty print output to a string.
/// If input and length are invalid or mismatched,　undefined behavior may occur.
/// The caller must ensure valid memory and the correct length.
/// Strings returned by bat_pretty_print_to_string are allocated by the library.
/// Call bat_free_string exactly once to free them after use.
/// Returns 0 on success, 1 on error.
#[no_mangle]
pub unsafe extern "C" fn bat_pretty_print_to_string(
    input: *const c_char,
    length: usize,
    input_type: BatInputType,
    language: *const c_char,
    theme: *const c_char,
    options: BatPrintOptions,
    output: *mut *const c_char,
    output_length: *mut usize,
) -> i32 {
    let language = if language.is_null() {
        None
    } else {
        match to_str(language) {
            Ok(lang) => Some(lang),
            Err(err) => {
                eprintln!("{}", err);
                return 1;
            }
        }
    };

    let theme = if theme.is_null() {
        None
    } else {
        Some(to_str(theme).unwrap())
    };

    let mut printer = match prepare_printer(input, length, input_type, language, theme, options) {
        Ok(printer) => printer,
        Err(err) => {
            eprintln!("{}", err);
            return 1;
        }
    };

    let mut output_str = String::new();
    if let Err(err) = printer.print_with_writer(Some(&mut output_str)) {
        eprintln!("{}", err);
        return 1;
    }

    let output_cstr = CString::new(output_str).unwrap();
    *output_length = output_cstr.to_bytes().len();
    *output = output_cstr.into_raw();

    0
}

/// Free the string allocated by `bat_pretty_print_to_string`.
#[no_mangle]
pub unsafe extern "C" fn bat_free_string(s: *const c_char) {
    if !s.is_null() {
        drop(CString::from_raw(s as *mut c_char));
    }
}

/// Return the version of the library
#[no_mangle]
pub extern "C" fn bat_c_version() -> *const c_char {
    static VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), "\0");
    VERSION.as_ptr() as *const c_char
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_bat_pretty_print_bytes() {
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

        let result = unsafe {
            bat_pretty_print(
                input_cstr.as_ptr(),
                input.len(),
                BatInputType::BatBytes,
                language_cstr.as_ptr(),
                std::ptr::null(),
                options,
            )
        };

        assert_eq!(result, 0);
    }

    #[test]
    fn test_bat_pretty_print_file() {
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

        let result = unsafe {
            bat_pretty_print(
                file_path_cstr.as_ptr(),
                0,
                BatInputType::BatFile,
                language_cstr.as_ptr(),
                std::ptr::null(),
                options,
            )
        };

        assert_eq!(result, 0);
    }

    #[test]
    fn test_bat_pretty_print_files() {
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

        let result = unsafe {
            bat_pretty_print(
                file_paths_ptr.as_ptr() as *const c_char,
                file_paths.len(),
                BatInputType::BatFiles,
                language_cstr.as_ptr(),
                std::ptr::null(),
                options,
            )
        };

        assert_eq!(result, 0);
    }

    #[test]
    fn test_bat_pretty_print_to_string() {
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

        let mut output_str = std::ptr::null();
        let mut output_length = 0;
        let result = unsafe {
            bat_pretty_print_to_string(
                input_cstr.as_ptr(),
                input.len(),
                BatInputType::BatBytes,
                language_cstr.as_ptr(),
                std::ptr::null(),
                options,
                &mut output_str,
                &mut output_length,
            )
        };

        assert_eq!(result, 0);
        assert_eq!(output_str.is_null(), false);

        // check if the output is a valid string
        let output = unsafe { CStr::from_ptr(output_str).to_str().unwrap() };
        assert_eq!(output.contains("Hello world!"), true);

        unsafe {
            bat_free_string(output_str);
        }
    }

    #[test]
    fn test_bat_c_version() {
        let version = unsafe { CStr::from_ptr(bat_c_version()).to_str().unwrap() };
        assert_eq!(version, env!("CARGO_PKG_VERSION"));
    }
}

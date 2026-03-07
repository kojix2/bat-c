use bat::{Input, PrettyPrinter};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::Path;
use std::slice;

/// Input type for FFI entry points.
#[repr(C)]
pub enum BatInputType {
    BatBytes,
    BatFile,
    BatFiles,
}

impl TryFrom<i32> for BatInputType {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(BatInputType::BatBytes),
            1 => Ok(BatInputType::BatFile),
            2 => Ok(BatInputType::BatFiles),
            _ => Err(format!(
                "invalid input_type: {} (expected 0=BatBytes, 1=BatFile, 2=BatFiles)",
                value
            )),
        }
    }
}

/// Print options.
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

fn to_str<'a>(cstr: *const c_char) -> Result<&'a str, std::str::Utf8Error> {
    unsafe { CStr::from_ptr(cstr).to_str() }
}

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

fn set_input(
    printer: &mut PrettyPrinter,
    input_type: BatInputType,
    input: *const c_char,
    length: usize,
) -> Result<(), String> {
    match input_type {
        BatInputType::BatBytes => {
            if input.is_null() {
                if length == 0 {
                    printer.input(Input::from_bytes(&[]));
                    return Ok(());
                }
                return Err("BatBytes input pointer is NULL while length > 0".into());
            }
            let input_str = unsafe {
                std::str::from_utf8(slice::from_raw_parts(input as *const u8, length))
                    .map_err(|e| e.to_string())?
            };
            printer.input(Input::from_bytes(input_str.as_bytes()));
        }
        BatInputType::BatFile => {
            if input.is_null() {
                return Err("BatFile input pointer is NULL".into());
            }
            let file_path = to_str(input).map_err(|e| e.to_string())?;
            printer.input_file(Path::new(file_path));
        }
        BatInputType::BatFiles => {
            if input.is_null() && length > 0 {
                return Err("BatFiles input pointer is NULL while length > 0".into());
            }

            let raw_paths = unsafe { slice::from_raw_parts(input as *const *const c_char, length) };
            let mut paths: Vec<&Path> = Vec::with_capacity(length);
            for (i, &ptr) in raw_paths.iter().enumerate() {
                if ptr.is_null() {
                    return Err(format!("BatFiles path at index {} is NULL", i));
                }
                let path = unsafe { CStr::from_ptr(ptr) }.to_str().map_err(|e| {
                    format!("BatFiles path at index {} is not valid UTF-8: {}", i, e)
                })?;
                paths.push(Path::new(path));
            }
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

/// Prints highlighted output.
///
/// # Safety
/// `input`, `language`, and `theme` must be valid pointers according to `input_type`.
#[no_mangle]
pub unsafe extern "C" fn bat_pretty_print(
    input: *const c_char,
    length: usize,
    input_type: i32,
    language: *const c_char,
    theme: *const c_char,
    options: BatPrintOptions,
) -> i32 {
    let input_type = match BatInputType::try_from(input_type) {
        Ok(input_type) => input_type,
        Err(err) => {
            eprintln!("{}", err);
            return 1;
        }
    };

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
        match to_str(theme) {
            Ok(thm) => Some(thm),
            Err(err) => {
                eprintln!("{}", err);
                return 1;
            }
        }
    };

    match execute_pretty_print(input, length, input_type, language, theme, options) {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("{}", err);
            1
        }
    }
}

/// Prints highlighted output into an allocated C string.
///
/// # Safety
/// Pointers must follow the API contract. Free `*output` with `bat_free_string`.
#[no_mangle]
pub unsafe extern "C" fn bat_pretty_print_to_string(
    input: *const c_char,
    length: usize,
    input_type: i32,
    language: *const c_char,
    theme: *const c_char,
    options: BatPrintOptions,
    output: *mut *const c_char,
    output_length: *mut usize,
) -> i32 {
    if output.is_null() || output_length.is_null() {
        eprintln!("output and output_length must be non-NULL");
        return 1;
    }

    let input_type = match BatInputType::try_from(input_type) {
        Ok(input_type) => input_type,
        Err(err) => {
            eprintln!("{}", err);
            return 1;
        }
    };

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
        match to_str(theme) {
            Ok(thm) => Some(thm),
            Err(err) => {
                eprintln!("{}", err);
                return 1;
            }
        }
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

    let output_cstr = match CString::new(output_str) {
        Ok(output_cstr) => output_cstr,
        Err(err) => {
            eprintln!("output contains an interior NUL byte: {}", err);
            return 1;
        }
    };
    *output_length = output_cstr.to_bytes().len();
    *output = output_cstr.into_raw();

    0
}

/// Frees a string returned by `bat_pretty_print_to_string`.
/// # Safety
/// `s` must be a live pointer returned by `bat_pretty_print_to_string`.
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

    fn test_options() -> BatPrintOptions {
        BatPrintOptions {
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
        }
    }

    #[test]
    fn test_bat_pretty_print_bytes() {
        let input = "<span style=\"color: #ff00cc\">Hello world!</span>\n";
        let input_cstr = CString::new(input).unwrap();
        let language_cstr = CString::new("html").unwrap();
        let options = test_options();

        let result = unsafe {
            bat_pretty_print(
                input_cstr.as_ptr(),
                input.len(),
                BatInputType::BatBytes as i32,
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
        let options = test_options();

        let result = unsafe {
            bat_pretty_print(
                file_path_cstr.as_ptr(),
                0,
                BatInputType::BatFile as i32,
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
        let options = test_options();

        let result = unsafe {
            bat_pretty_print(
                file_paths_ptr.as_ptr() as *const c_char,
                file_paths.len(),
                BatInputType::BatFiles as i32,
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
        let options = test_options();

        let mut output_str = std::ptr::null();
        let mut output_length = 0;
        let result = unsafe {
            bat_pretty_print_to_string(
                input_cstr.as_ptr(),
                input.len(),
                BatInputType::BatBytes as i32,
                language_cstr.as_ptr(),
                std::ptr::null(),
                options,
                &mut output_str,
                &mut output_length,
            )
        };

        assert_eq!(result, 0);
        assert!(!output_str.is_null());

        let output = unsafe { CStr::from_ptr(output_str).to_str().unwrap() };
        assert!(output.contains("Hello world!"));

        unsafe {
            bat_free_string(output_str);
        }
    }

    #[test]
    fn test_bat_c_version() {
        let version = unsafe { CStr::from_ptr(bat_c_version()).to_str().unwrap() };
        assert_eq!(version, env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn test_bat_pretty_print_invalid_input_type() {
        let input = CString::new("hello\n").unwrap();
        let result = unsafe {
            bat_pretty_print(
                input.as_ptr(),
                6,
                999,
                std::ptr::null(),
                std::ptr::null(),
                test_options(),
            )
        };

        assert_eq!(result, 1);
    }

    #[test]
    fn test_bat_pretty_print_to_string_null_output_ptrs() {
        let input = CString::new("hello\n").unwrap();
        let mut output_length = 0usize;
        let result = unsafe {
            bat_pretty_print_to_string(
                input.as_ptr(),
                6,
                BatInputType::BatBytes as i32,
                std::ptr::null(),
                std::ptr::null(),
                test_options(),
                std::ptr::null_mut(),
                &mut output_length,
            )
        };

        assert_eq!(result, 1);
    }
}

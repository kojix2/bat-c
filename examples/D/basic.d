import core.stdc.stdio : printf, fprintf, stderr;
import core.stdc.stdlib : exit;
import core.stdc.string : strlen;

extern(C) {
    enum BatInputType {
        BatBytes = 0,
        BatFile = 1,
        BatFiles = 2
    }
    
    struct BatPrintOptions {
        uint tab_width;
        ubyte colored_output;
        ubyte true_color;
        ubyte header;
        ubyte line_numbers;
        ubyte grid;
        ubyte rule;
        ubyte show_nonprintable;
        ubyte snip;
        size_t wrapping_mode;
        ubyte use_italics;
        size_t paging_mode;
        size_t highlight_line;
    }
    
    int bat_pretty_print_to_string(
        const char* input,
        size_t length,
        BatInputType input_type,
        const char* language,
        const char* theme,
        BatPrintOptions options,
        const char** output,
        size_t* output_length
    );
    
    void bat_free_string(const char* s);
}

void main() {
    const char* text = "<span>Hello</span>\n".ptr;
    
    BatPrintOptions opt = {
        tab_width: 4,
        colored_output: 1,
        true_color: 1,
        header: 0,
        line_numbers: 0,
        grid: 0,
        rule: 0,
        show_nonprintable: 0,
        snip: 1,
        wrapping_mode: 1,
        use_italics: 1,
        paging_mode: 0,
        highlight_line: 0
    };
    
    const char* output;
    size_t output_len;
    
    int ret = bat_pretty_print_to_string(
        text,
        strlen(text),
        BatInputType.BatBytes,
        "html".ptr,
        "Nord".ptr,
        opt,
        &output,
        &output_len
    );
    
    if (ret != 0) {
        fprintf(stderr, "error\n".ptr);
        exit(1);
    }
    
    import core.stdc.stdio : fwrite, stdout;
    fwrite(output, 1, output_len, stdout);
    bat_free_string(output);
}

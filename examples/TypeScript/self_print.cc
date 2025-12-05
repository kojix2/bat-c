#include <node_api.h>
#include "../../bat.h"

napi_value SelfPrint(napi_env env, napi_callback_info info)
{
    const char *file = "self_print.cc";

    BatPrintOptions opt = {
        .tab_width = 4,
        .colored_output = 1,
        .true_color = 1,
        .header = 0,
        .line_numbers = 1,
        .grid = 0,
        .rule = 0,
        .show_nonprintable = 0,
        .snip = 1,
        .wrapping_mode = 1,
        .use_italics = 1,
        .paging_mode = 0,
        .highlight_line = 0,
    };

    const char *out = NULL;
    size_t len = 0;

    int ret = bat_pretty_print_to_string(
        file, 0, (BatInputType)1, "cpp", "Nord", opt, &out, &len);

    napi_value result;
    if (ret != 0)
    {
        napi_create_string_utf8(env, "error", NAPI_AUTO_LENGTH, &result);
        return result;
    }

    napi_create_string_utf8(env, out, len, &result);
    bat_free_string(out);
    return result;
}

napi_value Init(napi_env env, napi_value exports)
{
    napi_value fn;
    napi_create_function(env, NULL, 0, SelfPrint, NULL, &fn);
    napi_set_named_property(env, exports, "selfPrint", fn);
    return exports;
}

NAPI_MODULE(NODE_GYP_MODULE_NAME, Init)

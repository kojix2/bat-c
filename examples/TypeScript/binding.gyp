{
  "targets": [
    {
      "target_name": "basic",
      "sources": ["basic.cc"],
      "include_dirs": ["../.."],
      "conditions": [
        ["OS=='mac'", {
          "libraries": ["<(module_root_dir)/../../target/release/libbat_c.dylib"],
          "xcode_settings": {
            "OTHER_LDFLAGS": ["-Wl,-rpath,<(module_root_dir)/../../target/release"]
          }
        }],
        ["OS=='linux'", {
          "libraries": ["<(module_root_dir)/../../target/release/libbat_c.so"],
          "ldflags": ["-Wl,-rpath=<(module_root_dir)/../../target/release"]
        }]
      ]
    },
    {
      "target_name": "self_print",
      "sources": ["self_print.cc"],
      "include_dirs": ["../.."],
      "conditions": [
        ["OS=='mac'", {
          "libraries": ["<(module_root_dir)/../../target/release/libbat_c.dylib"],
          "xcode_settings": {
            "OTHER_LDFLAGS": ["-Wl,-rpath,<(module_root_dir)/../../target/release"]
          }
        }],
        ["OS=='linux'", {
          "libraries": ["<(module_root_dir)/../../target/release/libbat_c.so"],
          "ldflags": ["-Wl,-rpath=<(module_root_dir)/../../target/release"]
        }]
      ]
    }
  ]
}

{
  "targets": [
    {
      "target_name": "basic",
      "sources": ["basic.cc"],
      "include_dirs": ["../.."],
      "libraries": [
        "-L../../target/release",
        "-lbat_c"
      ],
      "conditions": [
        ["OS=='mac'", {
          "xcode_settings": {
            "OTHER_LDFLAGS": ["-Wl,-rpath,@loader_path/../../target/release"]
          }
        }],
        ["OS=='linux'", {
          "ldflags": ["-Wl,-rpath=$ORIGIN/../../target/release"]
        }]
      ]
    },
    {
      "target_name": "self_print",
      "sources": ["self_print.cc"],
      "include_dirs": ["../.."],
      "libraries": [
        "-L../../target/release",
        "-lbat_c"
      ],
      "conditions": [
        ["OS=='mac'", {
          "xcode_settings": {
            "OTHER_LDFLAGS": ["-Wl,-rpath,@loader_path/../../target/release"]
          }
        }],
        ["OS=='linux'", {
          "ldflags": ["-Wl,-rpath=$ORIGIN/../../target/release"]
        }]
      ]
    }
  ]
}

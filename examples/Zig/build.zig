const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});

    // Basic example
    const basic_module = b.createModule(.{
        .root_source_file = b.path("basic.zig"),
        .target = target,
    });
    const exe = b.addExecutable(.{
        .name = "basic",
        .root_module = basic_module,
    });
    exe.addIncludePath(b.path("../.."));
    exe.addLibraryPath(b.path("../../target/release"));
    exe.linkSystemLibrary("bat_c");
    exe.linkLibC();
    b.installArtifact(exe);

    // Self_print example
    const self_print_module = b.createModule(.{
        .root_source_file = b.path("self_print.zig"),
        .target = target,
    });
    const exe2 = b.addExecutable(.{
        .name = "self_print",
        .root_module = self_print_module,
    });
    exe2.addIncludePath(b.path("../.."));
    exe2.addLibraryPath(b.path("../../target/release"));
    exe2.linkSystemLibrary("bat_c");
    exe2.linkLibC();
    b.installArtifact(exe2);
}

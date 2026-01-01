const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});

    // Basic example
    const basic_module = b.createModule(.{
        .root_source_file = b.path("basic.zig"),
        .target = target,
        .link_libc = true,
    });
    basic_module.addIncludePath(b.path("../.."));
    basic_module.addLibraryPath(b.path("../../target/release"));
    basic_module.linkSystemLibrary("bat_c", .{});
    
    const exe = b.addExecutable(.{
        .name = "basic",
        .root_module = basic_module,
    });
    b.installArtifact(exe);

    // Self_print example
    const self_print_module = b.createModule(.{
        .root_source_file = b.path("self_print.zig"),
        .target = target,
        .link_libc = true,
    });
    self_print_module.addIncludePath(b.path("../.."));
    self_print_module.addLibraryPath(b.path("../../target/release"));
    self_print_module.linkSystemLibrary("bat_c", .{});
    
    const exe2 = b.addExecutable(.{
        .name = "self_print",
        .root_module = self_print_module,
    });
    b.installArtifact(exe2);
}

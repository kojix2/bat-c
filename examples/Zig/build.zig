const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Dynamic linking examples
    const basic = b.addExecutable(.{
        .name = "basic",
        .root_source_file = b.path("basic.zig"),
        .target = target,
        .optimize = optimize,
    });
    basic.addIncludePath(b.path("../.."));
    basic.addLibraryPath(b.path("../../target/release"));
    basic.linkSystemLibrary("bat_c");
    basic.linkLibC();
    b.installArtifact(basic);

    const self_print = b.addExecutable(.{
        .name = "self_print",
        .root_source_file = b.path("self_print.zig"),
        .target = target,
        .optimize = optimize,
    });
    self_print.addIncludePath(b.path("../.."));
    self_print.addLibraryPath(b.path("../../target/release"));
    self_print.linkSystemLibrary("bat_c");
    self_print.linkLibC();
    b.installArtifact(self_print);

    // Run commands
    const run_basic = b.addRunArtifact(basic);
    const run_self_print = b.addRunArtifact(self_print);

    const run_basic_step = b.step("run-basic", "Run basic example");
    run_basic_step.dependOn(&run_basic.step);

    const run_self_print_step = b.step("run-self_print", "Run self_print example");
    run_self_print_step.dependOn(&run_self_print.step);
}

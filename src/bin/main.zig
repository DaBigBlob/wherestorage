//! The main executable. Used to upload files directly.
//!
const std = @import("std");
const lib = @import("wherestorage_lib");
const testing = std.testing;

pub fn main() !void {
    var gpa = std.heap.DebugAllocator(.{}){};
    defer _ = gpa.deinit();
    // const allocator = gpa.allocator();
    const foo: u16 = "";
    std.debug.print("{}", .{foo});
}

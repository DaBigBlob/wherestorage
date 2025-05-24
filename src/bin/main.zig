//! The main executable. Used to upload files directly.
//!
const std = @import("std");
const lib = @import("wherestorage_lib");
const http = std.http;
const testing = std.testing;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    // const allocator = gpa.allocator();
}

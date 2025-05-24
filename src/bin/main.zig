//! The main executable. Used to upload files directly.
//!
const std = @import("std");
const lib = @import("wherestorage_lib");
const testing = std.testing;
const http = std.http;

pub fn main() !void {
    var _alc = std.heap.ArenaAllocator{
        .child_allocator = std.heap.raw_c_allocator,
        .state = .{},
    };
    defer _ = _alc.deinit();
    const alc = _alc.allocator();

    // var client = http.Client{ .allocator = gpa.allocator() };

    const uri = comptime try std.Uri.parse("https://api.hman.io/headerdebug");
    std.debug.print("uri = {}\n", .{uri});

    const payload = try std.fmt.allocPrint(
        alc,
        "{{\"serverid\":{d},\"ping\":{d},\"upload\":{d},\"download\":{d},\"hash\":\"{s}\"}}",
        .{
            1897,
            69,
            69690,
            69690,
            "098680718fcd24abc8bafcbd3a802ad1",
        },
    );
    defer alc.free(payload);

    std.debug.print("payload = {s}\n", .{payload});
}

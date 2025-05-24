//! The main executable. Used to upload files directly.
//!
const std = @import("std");
const lib = @import("wherestorage_lib");
const http = std.http;
const testing = std.testing;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}).init;
    var client = http.Client{ .allocator = gpa.allocator() };
    const res = try http.Client.fetch(
        &client,
        .{
            .location = .{ .url = "https://0x000.io/1x1" },
            .method = .POST,
            .headers = .{
                .user_agent = .{ .override = "Mozilla/5.0 (Windows NT 10.0; rv:128.0) Gecko/20100101 Firefox/128.0" },
                .accept_encoding = .{ .override = "gzip, deflate, br, zstd" },
                .content_type = .{ .override = "application/json;charset=UTF-8" },
                .authorization = .omit,
                .connection = .{ .override = "keep-alive" },
                .host = .omit,
            },
        },
    );
    std.debug.print("{}\n", .{res.status});
}

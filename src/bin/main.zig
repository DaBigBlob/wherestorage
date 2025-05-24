//! The main executable. Used to upload files directly.
//!
const std = @import("std");
const lib = @import("wherestorage_lib");
const http = std.http;
const testing = std.testing;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}).init;
    var client = http.Client{
        .allocator = gpa.allocator(),
        .ca_bundle = std.crypto.Certificate.Bundle,
    };
    const res = try http.Client.fetch(
        &client,
        .{
            .location = .{ .url = "https://api.hman.io/headerdebug" },
            .method = .POST,
            .headers = .{
                .user_agent = .{ .override = "Mozilla/5.0 (Windows NT 10.0; rv:128.0) Gecko/20100101 Firefox/128.0" },
                .accept_encoding = .{ .override = "gzip, deflate, br, zstd" },
                .content_type = .{ .override = "application/json;charset=UTF-8" },
                .authorization = .omit,
                .connection = .default,
                .host = .omit,
            },
            .keep_alive = true,
            .payload = "{\"serverid\": 1897,\"ping\": 69,\"upload\": 69000,\"download\": 69000,\"hash\": \"098680718fcd24abc8bafcbd3a802ad1\"}",
        },
    );
    std.debug.print("{}\n", .{res.status});
}

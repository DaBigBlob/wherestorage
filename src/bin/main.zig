//! The main executable. Used to upload files directly.
//!
const std = @import("std");
const lib = @import("wherestorage_lib");
const http = std.http;
const testing = std.testing;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var client = http.Client{ .allocator = allocator };
    defer client.deinit();

    const uri = try std.Uri.parse("https://httpbin.org/anything");

    const payload =
        \\ {
        \\  "serverid": 1897,
        \\  "ping": 69,
        \\  "upload": 69000,
        \\  "download": 69000,
        \\  "hash": "098680718fcd24abc8bafcbd3a802ad1"
        \\ }
    ;

    var buf: [1024]u8 = undefined;
    var req = try client.open(
        .POST,
        uri,
        .{
            .server_header_buffer = &buf,
            .headers = .{
                .user_agent = .{ .override = "Mozilla/5.0 (Windows NT 10.0; rv:128.0) Gecko/20100101 Firefox/128.0" },
                .content_type = .{ .override = "application/json;charset=UTF-8" },
            },
        },
    );
    defer req.deinit();

    // req.transfer_encoding = .{ .content_length = payload.len };
    try req.send();
    // var wtr = req.writer();
    try req.writeAll(payload);
    try req.finish();
    try req.wait();

    // Occasionally, httpbin might time out, so we disregard cases
    // where the response status is not okay.
    if (req.response.status != .ok) {
        return;
    }

    var rdr = req.reader();
    const body = try rdr.readAllAlloc(allocator, 1024 * 1024 * 4);
    defer allocator.free(body);

    std.debug.print("Body:\n{s}\n", .{body});

    // var gpa = std.heap.GeneralPurposeAllocator(.{}).init;
    // defer _ = gpa.deinit();

    // var client = http.Client{ .allocator = gpa.allocator() };
    // defer _ = client.deinit();

    // var buf: [4096]u8 = undefined;

    // // const uri = try std.Uri.parse("https://api.hman.io/headerdebug");
    // var req = try client.open(
    //     .GET,
    //     try std.Uri.parse("https://api.hman.io/headerdebug"),
    //     .{
    //         .server_header_buffer = &buf,
    //         .headers = .{
    //             .user_agent = .{ .override = "Mozilla/5.0 (Windows NT 10.0; rv:128.0) Gecko/20100101 Firefox/128.0" },
    //             .content_type = .{ .override = "application/json;charset=UTF-8" },
    //         },
    //     },
    // );
    // defer _ = req.deinit();

    // try req.send();

    // _ = try req.writer().write("{\"serverid\": 1897,\"ping\": 69,\"upload\": 69000,\"download\": 69000,\"hash\": \"098680718fcd24abc8bafcbd3a802ad1\"}");

    // try req.finish();

    // try req.wait();

    // std.debug.print("status={d}\n", .{req.response.status});

    // const res = try http.Client.fetch(
    //     &client,
    //     .{
    //         .location = .{ .url = "https://api.hman.io/headerdebug" },
    //         .method = .POST,
    //         .headers = .{
    //             .user_agent = .{ .override = "Mozilla/5.0 (Windows NT 10.0; rv:128.0) Gecko/20100101 Firefox/128.0" },
    //             // .accept_encoding = .{ .override = "gzip, deflate, br, zstd" },
    //             .content_type = .{ .override = "application/json;charset=UTF-8" },
    //             // .authorization = .omit,
    //             // .connection = .default,
    //             // .host = .omit,
    //         },
    //         .keep_alive = true,
    //         .payload = "{\"serverid\": 1897,\"ping\": 69,\"upload\": 69000,\"download\": 69000,\"hash\": \"098680718fcd24abc8bafcbd3a802ad1\"}",
    //     },
    // );
    // std.debug.print("{}\n", .{res.status});
}

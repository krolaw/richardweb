const std = @import("std");

extern fn consoleLog(arg: u32) void;

const Point = struct {
    x: f64,
    y: f64,
};

var canvas_buffer: [][4]u8 = &[0][4]u8{};

export fn drawChaos(canvas_size_int: usize, points: usize, ratio: f64, rotate: f64, zoom: f64) [*]u8 {
    if (canvas_buffer.len != canvas_size_int * canvas_size_int) {
        std.heap.wasm_allocator.free(canvas_buffer);
        canvas_buffer = std.heap.wasm_allocator.alloc([4]u8, canvas_size_int * canvas_size_int) catch {
            consoleLog(1);
            @panic("Cannot allocate memory");
        };
    }

    const background = [4]u8{ 255, 255, 255, 255 };
    @memset(canvas_buffer, background);

    const pts = std.heap.wasm_allocator.alloc(Point, points) catch {
        consoleLog(1);
        @panic("Cannot allocate memory");
    };
    defer std.heap.wasm_allocator.free(pts);

    const canvas_size: f64 = @as(f64, @floatFromInt(canvas_size_int));

    const radius: f64 = zoom * canvas_size / 2.0;

    const centre = Point{
        .x = canvas_size / 2.0,
        .y = canvas_size / 2.0,
    };

    for (0.., pts) |i, *p| p.* = Point{
        .x = centre.x + radius * @sin(std.math.pi * 2 * @as(f64, @floatFromInt(i)) / @as(f64, @floatFromInt(pts.len))),
        .y = centre.y - radius * @cos(std.math.pi * 2 * @as(f64, @floatFromInt(i)) / @as(f64, @floatFromInt(pts.len))),
    };

    var pt = pts[0];

    var rnd = std.Random.DefaultPrng.init(0);
    const rot = rotate * std.math.pi / 180.0;
    const sin_rot = @sin(rot);
    const cos_rot = @cos(rot);

    for (pts) |p| canvas_buffer[@as(usize, @intFromFloat(p.y)) * canvas_size_int + @as(usize, @intFromFloat(p.x))] = [4]u8{ 0, 0, 0, 255 };

    var count: usize = 0;
    while (count < 100) {
        count += 1;
        const rpt = pts[@truncate(rnd.next() % pts.len)];
        pt = Point{
            .x = pt.x + (rpt.x - pt.x) * ratio,
            .y = pt.y + (rpt.y - pt.y) * ratio,
        };
        if (rotate != 0.0) {
            pt.x = (pt.x - rpt.x) * cos_rot - (pt.y - rpt.y) * sin_rot + rpt.x;
            pt.y = (pt.x - rpt.x) * sin_rot + (pt.y - rpt.y) * cos_rot + rpt.y;
        }
        if (pt.x < 0.1 or pt.y < 0.1 or pt.y > canvas_size or pt.x > canvas_size) {
            continue;
        }

        const pixel = &canvas_buffer[@as(usize, @intFromFloat(pt.y)) * canvas_size_int + @as(usize, @intFromFloat(pt.x))];

        if (std.mem.eql(u8, pixel, &background)) count = 0;
        pixel.* = [4]u8{ 0, 0, 0, 255 };
    }

    return @ptrCast(canvas_buffer);
}

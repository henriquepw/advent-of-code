const std = @import("std");
const utils = @import("utils.zig");

const allocator = std.heap.page_allocator;

fn getId(half: usize) !usize {
    const id = try std.fmt.allocPrint(allocator, "{d}{d}", .{ half, half });
    defer allocator.free(id);
    return std.fmt.parseInt(usize, id, 10);
}

fn getInvalidIds(head: []const u8, tail: []const u8) !usize {
    var sum: usize = 0;
    const is_head_even = head.len % 2 == 0;

    var start_haft: usize = 0;
    if (is_head_even) {
        start_haft = try std.fmt.parseInt(usize, head[0 .. head.len / 2], 10);
    } else {
        start_haft = std.math.pow(usize, 10, (head.len - 1) / 2);
    }

    if (start_haft == 0) {
        return sum;
    }

    var id: usize = 0;
    var count: usize = 0;
    const end = try std.fmt.parseInt(usize, tail, 10);
    const min = try std.fmt.parseInt(usize, head, 10);
    std.debug.print(" -- start {d} - end {d}\n", .{ start_haft, end });
    while (true) {
        id = try getId(start_haft);
        if (id > end) {
            break;
        }

        count += 1;
        start_haft += 1;
        if (id > min) {
            sum += id;
            std.debug.print(" -- {d} - id {d}\n", .{ count, id });
        }
    }

    return sum;
}

pub fn part1() !void {
    std.debug.print("Day 2:\n", .{});

    var input = try utils.readInput("inputs/02.txt");
    defer input.close();

    var result: usize = 0;
    var count: usize = 0;
    while (try input.next(',')) |range| {
        count += 1;
        var split = std.mem.splitScalar(u8, range, '-');
        const head = split.next() orelse unreachable;
        const tail = std.mem.trim(u8, split.next() orelse unreachable, "\t\n\r");

        std.debug.print("range {d} - {s} <> {s} \n", .{ count, head, tail });
        result += try getInvalidIds(head, tail);
    }

    std.debug.print("result {d}\n", .{result});
}

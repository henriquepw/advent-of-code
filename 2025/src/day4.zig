const std = @import("std");
const utils = @import("utils.zig");

const allocator = std.heap.page_allocator;

pub fn part1(path: []const u8) !usize {
    std.debug.print("Day 3:\n", .{});

    var input = try utils.readInput(path);
    defer input.close();

    while (try input.next('\n')) |line| {
        std.debug.print("{s}\n", .{line});
    }

    return 0;
}

pub fn part2(path: []const u8) !usize {
    std.debug.print("Day 3:\n", .{});

    var input = try utils.readInput(path);
    defer input.close();

    while (try input.next('\n')) |line| {
        std.debug.print("{s}\n", .{line});
    }

    return 0;
}

const expect = std.testing.expect;
test "day 4 part 1" {
    const result = try part1("inputs/04.test");
    try expect(result == 13);
}

test "day 4 part 2" {
    const result = try part2("inputs/04.test");
    try expect(result == 0);
}

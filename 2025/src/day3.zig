const std = @import("std");
const utils = @import("utils.zig");

const allocator = std.heap.page_allocator;

fn getJolts(max: usize, current: usize) !usize {
    const newJolts = try std.fmt.allocPrint(allocator, "{d}{d}", .{ max, current });
    return std.fmt.parseInt(usize, newJolts, 10);
}

pub fn part1(path: []const u8) !usize {
    std.debug.print("Day 3:\n", .{});

    var input = try utils.readInput(path);
    defer input.close();

    var result: usize = 0;

    while (try input.next('\n')) |bank| {
        var max: usize = 0;
        var jolts: usize = 0;
        for (bank) |byte| {
            const battery: usize = byte - '0';
            const newJolts = try getJolts(max, battery);
            if (newJolts > jolts) {
                jolts = newJolts;
            }
            if (battery > max) {
                max = battery;
            }
        }
        result += jolts;
    }

    std.debug.print(" -part 1 = {d}\n", .{result});
    return result;
}

pub fn part2() !void {
    // var input = try utils.readInput("inputs/02.txt");
    // defer input.close();
    //
    // var result: usize = 0;
    // while (try input.next(',')) |range| {
    // }

    std.debug.print(" -part 2 = \n", .{});
}

const expect = std.testing.expect;
test "day 3 part 1" {
    const result = try part1("inputs/03.test");
    try expect(result == 357);
}

const std = @import("std");
const utils = @import("utils.zig");

fn getSpin(line: []u8) !isize {
    const spin = try std.fmt.parseInt(isize, line[1..], 10);
    if (line[0] == 'L') {
        return -1 * spin;
    }

    return spin;
}

fn rotate(point: isize, value: isize) isize {
    const result = @mod(point + value, 100);
    if (result < 0) {
        return result + 100;
    }

    return result;
}

pub fn part1() !void {
    std.debug.print("Day 1:\n", .{});

    var input = try utils.readInput("inputs/01.txt");
    defer input.close();

    var result: usize = 0;
    var dial: isize = 50;
    while (try input.lines()) |line| {
        const spin = try getSpin(line);

        dial = rotate(dial, spin);
        if (dial == 0) {
            result += 1;
        }
    }

    std.debug.print(" -part 1 = {d}\n", .{result});
}

pub fn part2() !void {
    var input = try utils.readInput("inputs/01.txt");
    defer input.close();

    var result: isize = 0;
    var dial: isize = 50;
    while (try input.lines()) |line| {
        const spin = try getSpin(line);

        const total = dial + spin;
        if (total > 99) {
            result += @divTrunc(total, 100);
        }
        if (total <= 0) {
            result += @divTrunc(-total, 100);
            if (dial != 0) {
                result += 1;
            }
        }

        dial = rotate(dial, spin);
    }

    std.debug.print(" -part 2 = {d}\n", .{result});
}

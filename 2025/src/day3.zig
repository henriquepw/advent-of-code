const std = @import("std");
const utils = @import("utils.zig");

const allocator = std.heap.page_allocator;

fn getJolts(max: usize, current: usize) !usize {
    const newJolts = try std.fmt.allocPrint(allocator, "{d}{d}", .{ max, current });
    return std.fmt.parseInt(usize, newJolts, 10);
}

pub fn concatenateUsize(numbers: []const usize) !usize {
    var result: []u8 = "";
    for (numbers) |num| {
        result = try std.fmt.allocPrint(allocator, "{s}{d}", .{ result, num });
    }

    defer allocator.free(result);
    return std.fmt.parseInt(usize, result, 10);
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

pub fn part2(path: []const u8) !usize {
    var input = try utils.readInput(path);
    defer input.close();

    var result: usize = 0;
    const battery_size = 12;
    while (try input.next('\n')) |bank| {
        var max: usize = 0;
        var head: usize = 0;
        var batteries: [battery_size]usize = .{0} ** battery_size;
        for (0..battery_size) |b| {
            var next_idx = head;
            const tail: usize = bank.len - battery_size + b + 1;
            for (head..tail) |index| {
                const battery: usize = bank[index] - '0';
                if (battery > max) {
                    max = battery;
                    next_idx = index + 1;
                }

                if (max == 9) {
                    next_idx = index + 1;
                    break;
                }
            }

            batteries[b] = max;
            head = next_idx;
            max = 0;
        }

        result += try concatenateUsize(&batteries);
    }

    std.debug.print(" -part 2 = {d}\n", .{result});
    return result;
}

const expect = std.testing.expect;
test "day 3 part 1" {
    const result = try part1("inputs/03.test");
    try expect(result == 357);
}

test "day 3 part 2" {
    const result = try part2("inputs/03.test");
    try expect(result == 3121910778619);
}

const std = @import("std");
const utils = @import("utils.zig");

const allocator = std.heap.page_allocator;

pub fn generateInvalidId(num: usize, reps: usize) !usize {
    var result: []u8 = "";
    for (0..reps) |_| {
        result = try std.fmt.allocPrint(allocator, "{s}{d}", .{ result, num });
    }

    return std.fmt.parseInt(usize, result, 10);
}

pub fn getInitialValue(size: usize) usize {
    if (size <= 1) {
        return 1;
    }

    return std.math.pow(usize, 10, size - 1);
}

fn getInvalidIds(head: []const u8, tail: []const u8) !usize {
    var sum: usize = 0;
    const end = try std.fmt.parseInt(usize, tail, 10);
    const min = try std.fmt.parseInt(usize, head, 10);

    var start_haft: usize = 0;
    if (head.len % 2 == 0) {
        start_haft = try std.fmt.parseInt(usize, head[0 .. head.len / 2], 10);
    } else {
        start_haft = std.math.pow(usize, 10, (head.len - 1) / 2);
    }

    var id: usize = try generateInvalidId(start_haft, 2);
    while (id <= end) : (id = try generateInvalidId(start_haft, 2)) {
        if (id > min) {
            sum += id;
        }

        start_haft += 1;
    }

    return sum;
}

pub fn part1() !void {
    std.debug.print("Day 2:\n", .{});

    var input = try utils.readInput("inputs/02.txt");
    defer input.close();

    var result: usize = 0;
    while (try input.next(',')) |range| {
        var split = std.mem.splitScalar(u8, range, '-');
        const head = split.next() orelse unreachable;
        const tail = std.mem.trim(u8, split.next() orelse unreachable, "\t\n\r");

        result += try getInvalidIds(head, tail);
    }

    std.debug.print(" -part 1 = {d}\n", .{result});
}

const Range = struct {
    min: usize,
    max: usize,
};

pub fn part2() !void {
    var result: usize = 0;
    var input = try utils.readInput("inputs/02.txt");
    defer input.close();

    var ranges: std.ArrayList(Range) = .empty;
    defer ranges.deinit(allocator);

    while (try input.next(',')) |range| {
        var split = std.mem.splitScalar(u8, range, '-');
        const head = split.next() orelse unreachable;
        const tail = std.mem.trim(u8, split.next() orelse unreachable, "\t\n\r");
        const min = try std.fmt.parseInt(usize, head, 10);
        const max = try std.fmt.parseInt(usize, tail, 10);

        try ranges.append(allocator, .{
            .min = min,
            .max = max,
        });
    }

    const digits = 10;
    for (1..(digits / 2) + 1) |num_size| {
        const min = getInitialValue(num_size);
        const max = getInitialValue(num_size + 1);
        for (min..max) |num| {
            for (2..digits + 1) |final_size| {
                const reps = final_size / num_size;
                if (final_size % num_size != 0 or reps < 2) {
                    continue;
                }

                const id = try generateInvalidId(num, reps);
                for (ranges.items) |range| {
                    if (id >= range.min and id <= range.max) {
                        result += id;
                        break;
                    }
                }
            }
        }
    }

    std.debug.print(" -part 2 = {d}\n", .{result});
}

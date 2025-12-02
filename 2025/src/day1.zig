const std = @import("std");

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

    const file = try std.fs.cwd().openFile("inputs/01.txt", .{});
    defer file.close();

    const buffer = try std.heap.page_allocator.alloc(u8, 1024);
    defer std.heap.page_allocator.free(buffer);

    var reader = file.reader(buffer);

    var result: usize = 0;
    var dial: isize = 50;
    while (try reader.interface.takeDelimiter('\n')) |line| {
        const spin = try getSpin(line);

        dial = rotate(dial, spin);
        if (dial == 0) {
            result += 1;
        }
    }

    std.debug.print(" -part 1 = {d}\n", .{result});
}

pub fn part2() !void {
    const file = try std.fs.cwd().openFile("inputs/01.txt", .{});
    defer file.close();

    const buffer = try std.heap.page_allocator.alloc(u8, 1024);
    defer std.heap.page_allocator.free(buffer);

    var reader = file.reader(buffer);

    var result: isize = 0;
    var dial: isize = 50;
    while (try reader.interface.takeDelimiter('\n')) |line| {
        const spin = try getSpin(line);

        const total = dial + spin;
        if (total > 99) {
            result += @divTrunc(total, 100);
        }
        if (total <= 0) {
            const laps = @divTrunc(-total, 100) + 1;
            std.debug.print("total {d} - {d}\n", .{ total, laps });
            result += laps;
        }

        dial = rotate(dial, spin);
    }

    std.debug.print(" -part 2 = {d}\n", .{result});
}

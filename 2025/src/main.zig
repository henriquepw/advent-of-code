const std = @import("std");

const day1 = @import("day1.zig");
const day2 = @import("day2.zig");
const day3 = @import("day3.zig");

pub fn main() !void {
    // try day1.part1();
    // try day1.part2();
    // try day2.part1();
    // try day2.part2();
    _ = try day3.part1("inputs/03");
    _ = try day3.part2("inputs/03");
}

const std = @import("std");

const Input = struct {
    file: std.fs.File,
    buffer: []u8,
    reader: std.fs.File.Reader,
    pub fn next(self: *Input, delimiter: ?u8) error{ ReadFailed, StreamTooLong }!?[]u8 {
        return self.reader.interface.takeDelimiter(delimiter orelse '\n');
    }
    pub fn close(self: *Input) void {
        self.file.close();
        std.heap.page_allocator.free(self.buffer);
    }
};

pub fn readInput(src: []const u8) !Input {
    const file = try std.fs.cwd().openFile(src, .{});
    const buffer = std.heap.page_allocator.alloc(u8, 1024) catch |e| {
        file.close();
        return e;
    };

    const reader = file.reader(buffer);

    return .{
        .file = file,
        .buffer = buffer,
        .reader = reader,
    };
}

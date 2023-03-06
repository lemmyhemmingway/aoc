const std =  @import("std");
const data = @embedFile("day2.txt");

pub fn part1() !void {
  var lines = std.mem.tokenize(u8, data, "\n");

  // var up: i32 = 0;
  // var down: i32 = 0;
  while(lines.next()) |line| {
    std.debug.print("{s}", .{line});
  }
  // std.debug.print("{d}\n", .{down - up});
}

pub fn main() !void {
  try part1();
}

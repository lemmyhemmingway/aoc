const std =  @import("std");
const data = @embedFile("../../inputs/input.txt");

pub fn part1() !void {
  var lines = std.mem.tokenize(u8, data, "\n");

  var up: i32 = 0;
  var down: i32 = 0;
  while(lines.next()) |line| {
    // var splits = std.mem.split(u8, line, "");
    for (line) |character| {
      if (character == ')') {
        up += 1;
      } else {
        down += 1;
      }
    }
  }
  std.debug.print("{d}\n", .{down - up});
}

pub fn part2() !void {
  var lines = std.mem.tokenize(u8, data, "\n");

  var pos: i32 = 0;
  var i: i32 = 0;
  while(lines.next()) |line| {
    // var splits = std.mem.split(u8, line, "");
    for (line) |character| {
      i += 1;
      if (character == '(') {
        pos += 1;
      } else {
        pos -= 1;
      }
      if (pos == -1) {
        std.debug.print("{d}\n", .{i});
        return;
      }
    }
  }
}

pub fn main() !void {
  // try part1();
  try part2();
}

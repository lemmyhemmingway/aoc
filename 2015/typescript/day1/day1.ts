import * as fs from "fs";
function readFile(fileName:string): string[] {
  const data: Buffer = fs.readFileSync(fileName);
  return data.toString().split("")

}

function part1(): number {
  const inputs: string[] = readFile("../../inputs/input.txt");
  let up: number = 0;
  let down: number = 0;

  inputs.forEach((item, _) => {
    if (item === ")") {
      up += 1;
    } else if (item === "(") {
      down += 1;
    }
  })

  return down - up;

}

function part2(): number {
  const inputs: string[] = readFile("../../inputs/input.txt");
  let position: number = 0;

  for (let [index, value] of inputs.entries()) {
    if (value === ")") { position -= 1; }
    if (value === "(") { position += 1; }
    if (position === -1) {
      return index + 1;
    }
  }
  return -1

}

// console.log(part1());

console.log(part2())

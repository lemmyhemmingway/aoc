from typing import List


file_name = "../../inputs/day2.txt"
def read_file(file_name: str) -> List[str]:
    with open(file_name) as f:
        return f.readlines()

def part1() -> int:
    data = read_file(file_name)
    total = 0
    for line in data:
        l, w, h = [int(x) for x in line.strip().split("x")]
        area1 = l * w
        area2 = l * h
        area3 = h * w
        smallest_area = min([area1, area2, area3])
        total += area1 * 2 + area2 * 2 + area3 * 2 + smallest_area

    return total


def part2() -> int:
    data = read_file(file_name)
    total = 0
    for line in data:
        l, w, h = sorted([int(x) for x in line.strip().split("x")])
        wrap = l * 2 + w * 2
        bow = l * w * h

        total += wrap + bow

    return total



if __name__ == "__main__":
    print(part1())
    print(part2())


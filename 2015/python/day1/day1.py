from typing import List


def read_file(file_name: str) -> List[str]:
    with open(file_name) as f:
        data = f.read()
        return [x for x in list(data)]

def part1() -> int:
    data: List[str] = read_file("../../inputs/input.txt")
    up: int = 0
    down: int = 0
    for item in data:
        if item == ")":
            up += 1
        elif item == "(":
            down += 1
    return down - up


def part2() -> int:
    data: List[str] = read_file("../../inputs/input.txt")

    position: int = 0
    for index, item in enumerate(data):
        if item == ")":
            position -= 1
        elif item == "(":
            position += 1
        if position == -1:
            return index + 1
    return 0


def main():
    # print(part1())
    print(part2())

if __name__ == "__main__":
    main()


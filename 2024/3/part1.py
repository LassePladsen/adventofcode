from functools import reduce
import re


def read_input() -> str:
    with open("input", "r") as file:
        return file.read()


def main():
    matches = re.findall(r"mul\((\d{1,3}),(\d{1,3})\)", read_input())
    sum = reduce(lambda sum, vals: sum + int(vals[0]) * int(vals[1]), matches, 0)
    print("Total sum is", sum)


if __name__ == "__main__":
    main()

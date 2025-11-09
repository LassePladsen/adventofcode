from functools import reduce
import re
from part1 import read_input


def main():
    input = read_input()
    instructions = re.findall(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)", input)
    do = True
    sum = 0
    digit_pattern = re.compile(r"(\d{1,3})")
    for instruction in instructions:
        match instruction:
            case "do()":
                do = True
            case "don't()":
                do = False
            case _:
                if do:
                    sum += reduce(
                        lambda prod, val: prod * int(val),
                        digit_pattern.findall(instruction),
                        1,
                    )

    print("Total sum is", sum)


if __name__ == "__main__":
    main()

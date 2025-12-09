import sys
import re


DEBUG = False


def debugprint(msg):
    if DEBUG:
        print(msg)


def read_filepath():
    return sys.argv[1] if len(sys.argv) >= 2 else "input.txt"


def read_input():
    filepath = read_filepath()
    with open(filepath, "r") as f:
        return f.read()


def read_id_ranges():
    return [s.strip() for s in read_input().split(",")]


def valid(id):
    # save regex time by knowing its valid early if the string length is odd
    s = str(id)
    if len(s) % 2 == 2:
        return True

    p = re.compile(r"^(\d+)\1$")
    captures = p.search(s)
    if captures:
        groups = captures.groups()
        debugprint(f"We got {len(groups)} matches: {groups}")
        return len(groups) == 2

    else:
        debugprint("No captures, its valid!")
        return True


def main():
    id_ranges = read_id_ranges()
    debugprint(f"LP id_ranges: {id_ranges}")
    invalid_sum = 0

    for rng in id_ranges:
        (start, end) = rng.split("-")
        debugprint(f"The range is {start}-{end}")

        # Loop over every id in the range, check for invalids
        for id in range(int(start), int(end) + 1):
            debugprint(f"id is {id}")
            if not valid(id):
                debugprint(f"Id {id} is invalid! Adding to sum")
                invalid_sum += id
                debugprint(f"LP new invalid_sum: {invalid_sum}")

    print(f"Total sum of invalid ids: {invalid_sum}")


if __name__ == "__main__":
    main()

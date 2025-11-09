from functools import reduce

lists = tuple[list[int], list[int]]


def read_lists() -> lists:
    left_list: list[int] = []
    right_list: list[int] = []
    with open("input", "r") as file:
        lines = file.readlines()
        for line in lines:
            left, right = line.split()
            left_list.append(int(left))
            right_list.append(int(right))
    return left_list, right_list


def calculate_total_distance(lists: lists) -> int:
    return reduce(
        lambda sum, vals: sum + abs(vals[0] - vals[1]), zip(lists[0], lists[1]), 0
    )


def main():
    lists = read_lists()
    lists[0].sort()
    lists[1].sort()
    total_distance = calculate_total_distance(lists)
    print("total_distance: ", total_distance)


if __name__ == "__main__":
    main()

from part1 import lists, read_lists


def calculate_total_similarity(lists: lists) -> int:
    total = 0
    for left_val in lists[0]:
        total += lists[1].count(left_val) * left_val
    return total


def main():
    lists = read_lists()
    print("Total similary score:", calculate_total_similarity(lists))


if __name__ == "__main__":
    main()

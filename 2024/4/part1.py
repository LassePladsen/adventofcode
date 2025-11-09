import numpy as np

input_type = np.ndarray

DEBUG = False


def read_input() -> input_type:
    with open("input", "r") as file:
        lines = []
        for line in file:
            lines.append(list(line.strip()))
        return np.array(lines)


def string_count(input: input_type, target: str):
    count = 0
    length = len(target)

    rows, cols = input.shape
    if DEBUG:
        print(f"{rows=}, {cols=}")

    # Loop over every single char...
    for row in range(rows):
        if DEBUG:
            print(row, input[row])
        for col in range(cols):
            if DEBUG:
                print("col=", col)
            # Horizontal left
            if col - length + 1 >= 0:
                # left = "".join(input[row, col - length : col][::-1])
                left = "".join([input[row, col - i] for i in range(length)])
                if left == target:
                    count += 1

            # Horizontal right
            if col + length < cols:
                # right = "".join(input[row, col : col + length])
                right = "".join([input[row, col + i] for i in range(length)])
                if right == target:
                    count += 1

            # Vertical down
            if row + length < rows:
                # down = "".join(input[row : row + length, col])
                down = "".join([input[row + i, col] for i in range(length)])
                if down == target:
                    count += 1

            # Vertical up
            if row - length + 1 >= 0:
                # up = "".join(input[row - length + 1 : row + 1, col][::-1])
                up = "".join([input[row - i, col] for i in range(length)])
                if up == target:
                    count += 1

            # Diagonal up left
            if col - length + 1 >= 0 and row - length + 1 >= 0:
                left_up = "".join([input[row - i, col - i] for i in range(length)])
                if left_up == target:
                    count += 1

            # Diagonal up right
            if col + length < cols and row - length + 1 >= 0:
                up_right = "".join([input[row - i, col + i] for i in range(length)])
                if up_right == target:
                    count += 1

            # Diagonal down right
            if col + length < cols and row + length < rows:
                down_right = "".join([input[row + i, col + i] for i in range(length)])
                if down_right == target:
                    count += 1

            # Diagonal down left
            if col - length + 1 >= 0 and row + length < rows:
                down_left = "".join([input[row + i, col - i] for i in range(length)])
                if down_left == target:
                    count += 1

    return count


def main() -> None:
    print("Count of XMAS:", string_count(read_input(), "XMAS"))
    print("2310 IS WRONG ANSWER! https://adventofcode.com/2024/day/4")


if __name__ == "__main__":
    main()

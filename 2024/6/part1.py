from functools import reduce
from pprint import pprint
from typing import Any, Literal

DEBUG_LEVEL = 0
GUARD_CHARS = ["^", ">", "<", "v"]
EMPTY_CHAR = "."
OBSTACLE_CHAR = "#"
VISITED_CHAR = "X"

Direction = Literal["left", "right", "up", "down"]
Guard = Literal["^", ">", "<", "v"]
Char = Guard | Literal[".", "#", "X"]
Map = list[list[Char]]


def debug_print(msg: Any, level: int = 1, pretty_print: bool = False) -> None:
    if DEBUG_LEVEL > level:
        if pretty_print:
            pprint(msg)
            return
        print(msg)


class GuardNotFoundException(Exception):
    pass


class MapSolver:
    _char_direction_map: dict[Guard, Direction] = {
        "^": "up",
        ">": "right",
        "<": "left",
        "v": "down",
    }
    is_solved: bool = False  # solved when guard leaves the map

    # flag to show if we already had visited this current coordinate
    _had_visited_coord_before: bool = False
    _count_distinct_positions: int = 0
    guard_direction: Direction
    guard_coordinates: tuple[int, int]
    map: Map

    def __init__(self, map: Map) -> None:
        self.map = map

        # Find guard location
        for row, chars in enumerate(self.map):
            for col, char in enumerate(chars):
                # If no direction found, this is NOT a guard! Skip it
                if char not in self._char_direction_map:
                    continue
                direction = self._char_direction_map[char]
                self.guard_direction = direction
                self.guard_coordinates = (row, col)
                return
        raise GuardNotFoundException(f"Guard not found in the map {self.map}")

    def _maybe_increment_distinct_positions(self) -> None:
        """Increments distint position count if we haven't visited this position before"""
        if not self._had_visited_coord_before:
            self._count_distinct_positions += 1

    def _turn_right(self) -> None:
        match self.guard_direction:
            case "left":
                self.guard_direction = "up"
                self.set_char(self.guard_coordinates, "^")
            case "right":
                self.guard_direction = "down"
                self.set_char(self.guard_coordinates, "v")
            case "up":
                self.guard_direction = "right"
                self.set_char(self.guard_coordinates, ">")
            case "down":
                self.guard_direction = "left"
                self.set_char(self.guard_coordinates, "<")

    def _move_guard(self, coords: tuple[int, int]) -> None:
        debug_print(f"_move_guard() called with next coords={coords}", 2)
        guard_row, guard_col = self.guard_coordinates
        guard_char = self.map[guard_row][guard_col]

        self._had_visited_coord_before = self.map[coords[0]][coords[1]] == VISITED_CHAR
        debug_print(
            f"Before we moved, did we already visit this new position?={self._had_visited_coord_before}"
        )

        self._maybe_increment_distinct_positions()
        self.set_char((guard_row, guard_col), VISITED_CHAR)
        self.set_char((coords[0], coords[1]), guard_char)
        self.guard_coordinates = coords

    def set_char(self, coords: tuple[int, int], new_char: Char) -> None:
        self.map[coords[0]][coords[1]] = new_char

    @property
    def count_distinct_positions(self) -> int:
        """The count_distinct_positions property."""
        return self._count_distinct_positions

    def move(self) -> None:
        """Moves the guard one step, or turns if facing an obstacle"""
        debug_print(
            f"move() called, is_solved={self.is_solved}, coords={self.guard_coordinates}, direction={self.guard_direction}, distinct position count={self.count_distinct_positions}",
            2,
        )
        if self.is_solved:
            return

        # Calculate next coords in the correct direction
        guard_row, guard_col = self.guard_coordinates
        match self.guard_direction:
            case "left":
                next_row = guard_row
                next_col = guard_col - 1
            case "right":
                next_row = guard_row
                next_col = guard_col + 1
            case "up":
                next_row = guard_row - 1
                next_col = guard_col
            case "down":
                next_row = guard_row + 1
                next_col = guard_col

        # Moving out of bounds
        if (
            next_row < 0
            or next_col < 0
            or next_row > len(self.map) - 1
            or next_col > len(self.map[0]) - 1
        ):
            self.set_char((guard_row, guard_col), VISITED_CHAR)
            self._maybe_increment_distinct_positions()
            self.is_solved = True
            return

        # Hitting an obstacle
        if self.map[next_row][next_col] == OBSTACLE_CHAR:
            self._turn_right()
            return

        self._move_guard((next_row, next_col))

    def solve(self, max_iters: int | None = None) -> None:
        """Runs the map solver for max_iters iterations, or until its solved if None given."""
        iters = 0
        while not self.is_solved:
            if max_iters is not None and iters >= max_iters:
                debug_print(f"Reached max iterations of {max_iters}, stopping...")
                return
            debug_print(f"New iteration, {iters=}")
            debug_print(self.print_map(True))
            self.move()
            iters += 1
        debug_print(f"Solved after {iters} iterations")

    def print_map(self, return_: bool = False) -> str | None:
        string = reduce(lambda string, row: f"{string}{''.join(row)}\n", self.map, "")

        if return_:
            return string
        print(string)


def read_input(filename: str = "input") -> Map:
    lines: Map = []
    with open(filename) as f:
        for line in f:
            lines.append(list(line.strip()))  # type: ignore
    return lines


def main() -> None:
    input_ = read_input()
    solver = MapSolver(input_)
    debug_print(solver.guard_coordinates)
    solver.solve()
    print("\nSolved/stopped map:")
    solver.print_map()
    print(f"Visited {solver._count_distinct_positions} distinct positions")


if __name__ == "__main__":
    main()

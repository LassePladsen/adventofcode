from part1 import read_input, update_follows_rule, Update, Rule

DEBUG_LEVEL = 10


def debug_print(*args, level: int = 1, **kwargs):
    if level <= DEBUG_LEVEL:
        print(*args, **kwargs)


def get_unfollowed_rules(update: Update, rules: list[Rule]) -> list[Rule]:
    unfollowed_rules: list[Rule] = []
    for rule in rules:
        if update_follows_rule(update, rule):
            continue
        debug_print(f"Update {update} did not follow {rule=}", level=2)
        unfollowed_rules.append(rule)
    return unfollowed_rules


def get_update_index_increments(update: Update, rules: list[Rule]) -> dict[int, int]:
    """Returns a dict mapping how many places to move a number in updates, from {index_in_update: places_to_move}"""
    index_increments: dict[int, int] = {}

    for rule in rules:
        index_increment = 0
        for index, num in enumerate(update):
            pass
        if index_increment == 0:
            debug_print(
                f"{num=} in {update=} got index increment of zero for {rule=}, continuing",
                level=3,
            )
            continue
        index_increments.update({index: index_increment})
    return index_increments


def reorder_update(update: Update, rules: list[Rule]) -> Update:
    index_increments = get_update_index_increments(update, rules)
    new_update = update.copy()
    return new_update


def maybe_reorder_update(update: Update, rules: list[Rule]) -> Update | None:
    """Returns None if update followed all rules"""
    unfollowed_rules = get_unfollowed_rules(update, rules)
    if len(unfollowed_rules) == 0:
        return None
    debug_print(f"{update=} does NOT follow all rules, reordering it", level=2)
    new_update = reorder_update(update, unfollowed_rules)
    debug_print(f"After reordering update is now {update}", level=2)
    return None


def main():
    input = read_input("testinput")
    debug_print(f"{str(input)}", level=2)
    exit()
    middle_sum = 0
    for update in input.updates:
        debug_print(f"Checking {update=}", level=2)
        maybe_update = maybe_reorder_update(update, input.rules)
        if maybe_update is None:
            debug_print(f"maybe_reorder_update returned None for {update=}", level=2)
            continue
        middle_sum += int(new_update[int(len(new_update) / 2)])
    print(f"Middle sum of incorrect *reordered* updates is {middle_sum}")


if __name__ == "__main__":
    main()

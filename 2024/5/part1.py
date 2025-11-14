from dataclasses import dataclass
from functools import reduce

Rule = tuple[str, str]
Update = list[str]

DEBUG_LEVEL = 0


@dataclass
class Input:
    rules: list[Rule]
    updates: list[Update]

def debug_print(*args, level: int = 1, **kwargs):
    if level <= DEBUG_LEVEL:
        print(*args, **kwargs)


def read_input(filename: str = "input") -> Input:
    input = Input([], [])
    with open(filename, "r") as file:
        section = 1
        for line in file:
            line = line.strip()
            if not line:
                section = 2
                continue

            if section == 1:
                input.rules.append(tuple(line.strip().split("|")))
            elif section == 2:
                input.updates.append(line.strip().split(","))
    return input


def update_follows_rule(update: Update, rule: Rule) -> bool:
    debug_print(f"Checking {rule=}...", level=2)
    first_matched = False
    second_matched = False
    for num in update:
        debug_print(f"Checking {num=}", level=3)
        if num not in rule:
            debug_print(f"Num {num} not in this rule, continuing early", level=3)
            continue
        if num == rule[0]:
            debug_print("First matched!", level=2)
            if second_matched:
                debug_print(
                    "Already had second match, update does NOT follow rule", level=2
                )
                return False
            first_matched = True
        elif num == rule[1]:
            debug_print("Second matched!", level=2)
            if first_matched:
                debug_print("We already had first, update follows rule", level=3)
                return True
            # debug_print(
            #     "Did not have first match, update does NOT follow rule", level=3
            # )
            debug_print("Did not have first match already", level=3)
            second_matched = True
    # exit(f"Hit fallback after loop in update_follows_rule for {update=} and {rule}...")
    debug_print("Hitting fallback, returning true!", level=3)
    return True


def unfollowed_rules_for_update(update: Update, rules: list[Rule]) -> bool:
    for rule in rules:
        if not update_follows_rule(update, rule):
            return False
        debug_print(f"{update=} follows {rule=}!", level=2)
    return True


def main():
    input = read_input()
    debug_print(f"{input=}", level=2)
    middle_sum = 0
    for update in input.updates:
        debug_print(f"Checking {update=}", level=2)
        if not unfollowed_rules_for_update(update, input.rules):
            continue
        debug_print(f"Update {update} follows rules!")
        middle_sum += int(update[int(len(update) / 2)])
    print(f"Middle sum of correct updates is {middle_sum}")


if __name__ == "__main__":
    main()

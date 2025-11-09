from part1 import report_type, level_type, read_reports

DEBUG = True


def is_level_safe(
    level: level_type, prev: level_type, prev_increment: int | None
) -> bool:
    if DEBUG:
        print(f"is_level_safe called with {prev=}, {level=}, {prev_increment=}")
    new_increment = level - prev
    if DEBUG:
        print("new_increment=", new_increment)

    # If product between last and this new increment is negative,
    # then their signs don't match (negative vs positive), so its unsafe. Also unsafe it its zero
    if prev_increment is not None and new_increment * prev_increment <= 0:
        if DEBUG:
            print("early return, level was UNSAFE")
        return False

    result = 1 <= abs(new_increment) <= 3
    if DEBUG:
        print("Level was safe?:", result)
    return result


def is_report_safe(report: report_type) -> bool:
    prev = report[0]
    prev_increment: int | None = None
    num_unsafe = 0
    for level in report[1:]:
        if not is_level_safe(level, prev, prev_increment):
            # We can handle just one safe level for the report to still be safe
            # FIXME: problem is that we now always skip the one that was unsafe, but we also have to check if its safe instead when skipping the previous one 
            num_unsafe += 1
            if num_unsafe > 1:
                return False

        else:
            prev_increment = level - prev
            prev = level
    return True


def calc_num_safe_reports(reports: list[report_type]) -> int:
    num_safe_reports = 0
    for report in reports:
        if DEBUG: print("\nNEW REPORT=", report)
        safe = is_report_safe(report)
        if DEBUG: print('Report was safe?: ', safe)
        num_safe_reports += int(safe)
    return num_safe_reports


def main():
    reports = read_reports()
    num_safe_reports = calc_num_safe_reports(reports)
    print("num_safe_reports: ", num_safe_reports)
    print("number of unsafe reports: ", len(reports) - num_safe_reports)


if __name__ == "__main__":
    main()

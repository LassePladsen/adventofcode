level_type = int
report_type = list[level_type]


def read_reports() -> list[report_type]:
    reports: list[report_type] = []
    with open("input", "r") as file:
        line = file.readline().strip()
        while line:
            if len(line) < 2:
                line = file.readline()
                continue
            reports.append([int(val) for val in line.split(" ")])
            line = file.readline()
    return reports


def is_level_safe(
    level: level_type, prev: level_type, prev_increment: int | None
) -> bool:
    new_increment = level - prev

    # If product between last and this new increment is negative,
    # then their signs don't match (negative vs positive), so its unsafe. Also unsafe it its zero
    if prev_increment is not None and new_increment * prev_increment <= 0:
        return False

    result = 1 <= abs(new_increment) <= 3
    return result


def is_report_safe(report: report_type) -> bool:
    prev = report[0]
    prev_increment: int | None = None
    for level in report[1:]:
        if not is_level_safe(level, prev, prev_increment):
            return False
        prev_increment = level - prev
        prev = level
    return True


def calc_num_safe_reports(reports: list[report_type]) -> int:
    num_safe_reports = 0
    for report in reports:
        safe = is_report_safe(report)
        num_safe_reports += int(safe)
    return num_safe_reports


def main():
    reports = read_reports()
    num_safe_reports = calc_num_safe_reports(reports)
    print("num_safe_reports: ", num_safe_reports)
    print("number of unsafe reports: ", len(reports) - num_safe_reports)


if __name__ == "__main__":
    main()

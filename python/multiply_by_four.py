import sys


def main() -> int:
    if len(sys.argv) != 2:
        print("Expected exactly one numeric argument", file=sys.stderr)
        return 1

    try:
        value = float(sys.argv[1])
    except ValueError:
        print("Argument must be a number", file=sys.stderr)
        return 1

    print(value * 4)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

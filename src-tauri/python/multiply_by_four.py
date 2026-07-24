from __future__ import annotations

import sys


def main() -> None:
    if len(sys.argv) != 2:
        raise SystemExit("Expected one numeric argument")

    value = float(sys.argv[1])
    print(value * 4)


if __name__ == "__main__":
    main()

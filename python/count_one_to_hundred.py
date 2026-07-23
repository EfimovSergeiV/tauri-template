import time


def main() -> int:
    for number in range(1, 101):
        print(number, flush=True)
        time.sleep(1)

    return 0


if __name__ == "__main__":
    raise SystemExit(main())

from __future__ import annotations

import json
import random


class User:
    def __init__(self, name: str, year: int, city: str) -> None:
        self.name = name
        self.year = year
        self.city = city

    def to_dict(self) -> dict:
        return {
            "name": self.name,
            "year": self.year,
            "city": self.city,
        }


def generate_users(count: int = 15) -> list[User]:
    names = [
        "Алексей", "Сергей", "Иван", "Дмитрий", "Андрей",
        "Максим", "Евгений", "Никита", "Павел", "Михаил",
        "Анна", "Мария", "Елена", "Ольга", "Дарья",
        "Екатерина", "Юлия", "Татьяна", "Алина", "Виктория",
    ]

    cities = [
        "Москва",
        "Санкт-Петербург",
        "Казань",
        "Новосибирск",
        "Екатеринбург",
        "Самара",
        "Краснодар",
        "Нижний Новгород",
        "Воронеж",
        "Челябинск",
    ]

    users: list[User] = []

    for _ in range(count):
        users.append(
            User(
                name=random.choice(names),
                year=random.randint(1970, 2010),
                city=random.choice(cities),
            )
        )

    return users


def main() -> None:
    users = generate_users()

    print(
        json.dumps(
            [user.to_dict() for user in users],
            ensure_ascii=False,
            indent=4,
        )
    )


if __name__ == "__main__":
    main()
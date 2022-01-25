from __future__ import annotations

import json
import logging
import typing
from dataclasses import dataclass
from functools import cache

logger = logging.getLogger(__name__)


@cache
def str_dist(
    source: str,
    target: str,
    clear_cache: bool = True,
) -> int:
    if source == "" or target == "":
        return len(source) + len(target)
    if target[0] == source[0]:
        return str_dist(source[1:], target[1:], clear_cache=False)

    result = 1 + min(
        # Delete one char from first string
        str_dist(source[1:], target, clear_cache=False),
        # Delete one char from second string
        str_dist(source, target[1:], clear_cache=False),
        # Change one char
        str_dist(source[1:], target[1:], clear_cache=False),
    )
    if clear_cache:
        # Clear cache to prevent memory leaks
        str_dist.cache_clear()
    return result


def match(source: str, target: str) -> float:
    dist = str_dist(source, target)
    str_dist.cache_clear()
    return 1.0 - dist / max(len(source), len(target), 1)


class AccountMatcher:
    def __init__(self, accounts: dict[str, str] = None):
        self.accounts: dict[str, str] = accounts or {}

    def ask_user_to_pick(self, account: str, candidates: list[(float, str)]) -> str:
        print(f"\n=== {account} ===")
        if not len(candidates):
            print("Please enter a new match for this account: ", end="")
            return input()

        for i, (score, m) in enumerate(candidates):
            print(f"{i+1}. {score*100:.1f}% ({self.accounts[m]}) {m}")
        print("Enter one of the numbers above or a new match for this account")
        return input(f"[1-{len(candidates)}]: ")

    def manual_match(self, account: str, candidates: list[(float, str)]) -> str:
        response = self.ask_user_to_pick(account, candidates)
        if response.isdigit() and 0 <= (idx := int(response) - 1) < len(candidates):
            self.accounts[account] = self.accounts[candidates[idx][1]]
        else:
            self.accounts[account] = response

        return self.accounts[account]

    def find_match(self, account: str) -> str:
        matches = [
            (score, acc)
            for acc in self.accounts
            if (score := match(account, acc)) > 0.5
        ]

        matches.sort()
        if len(matches) and (best := matches[-1]) and best[0] > 0.75:
            self.accounts[account] = self.accounts[best[1]]
            logger.info(
                f"Automatically matched '{self.accounts[account]}' to '{account}' ({best[0]*100:.1f})"
            )
            return self.accounts[account]

        return self.manual_match(account, matches)

    def dump(self, filename: str):
        logger.info(
            f"Writing matching data ({len(self.accounts)} entries) into {filename}."
        )
        with open(filename, "w") as store:
            json.dump(self.accounts, store)

    @classmethod
    def load(cls, filename: typing.Optional[str]) -> AccountMatcher:
        if filename is None:
            return cls()
        logger.info(f"Loading matching data into {filename}.")
        with open(filename, "r") as store:
            return cls(json.load(store))

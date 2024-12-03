from typing import Tuple, List
from pathlib import Path


def main():
    """
    Input is a file of two lists
    """

    list1, list2 = parse_input()
    sim_score = 0
    for i in list1:
        multiplier = 0
        for n in list2:
            if n == i:
                multiplier += 1
        sim_score += multiplier * i
    print(sim_score)


def parse_input(input_path: Path = "./input.txt") -> Tuple[List, List]:
    if isinstance(input_path, str):
        input_path = Path(input_path)
    list1, list2 = [], []
    with open(input_path) as f:
        for line in f:
            num1, num2 = line.split()
            list1.append(int(num1.strip()))
            list2.append(int(num2.strip()))
    return list1, list2


main()

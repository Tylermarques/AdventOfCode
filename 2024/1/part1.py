from typing import Tuple, List
from pathlib import Path
from loguru import logger


def main():
    """
    Input is a file of two lists
    """

    list1, list2 = parse_input()
    list1 = sorted(list1)
    list2 = sorted(list2)
    distance = 0
    list1_index = 0
    list2_index = 0
    total_iter = 0
    while True:
        i = list1[list1_index]
        n = list2[list2_index]
        distance += abs(i - n)
        if list1_index < len(list1):
            list1_index += 1
        if list2_index < len(list2):
            list2_index += 1
        total_iter += 1
        if total_iter >= max(len(list1), len(list2)):
            logger.warning("Reached max iterations")
            break
    print(distance)


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

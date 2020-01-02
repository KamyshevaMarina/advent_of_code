import re
import sys


def find_items():
    d = []
    with open("tadam.txt") as file_handler:
        for line in file_handler:
            d += line.replace('.', '').split()
    return d


def tree_creater(d, answer):

    child = int(d.pop(0))
    metdata = int(d.pop(0))
    pr = 0
    if child != 0:
        for i in range(child):
            answer = tree_creater(d, answer)
         ##   print(answer)
    elif child == 0:
        for i in range(metdata):
            answer += int(d.pop(0))
            pr = 1
        return answer
    if metdata != 0 and pr == 0:
        for i in range(metdata):
            answer += int(d.pop())

    return answer


if __name__ == "__main__":
    sys.setrecursionlimit(15000)
    print(tree_creater(find_items(), 0))

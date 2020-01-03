import re
import sys


def find_items():
    d = []
    with open("tadam.txt") as file_handler:
        for line in file_handler:
            d += line.replace('.', '').split()
    d = [int(i) for i in d]
    ##  print(d)
    ## print(len(d))
    return d


def tree_creater(d, answer):
    #print(d)
    child = d.pop(0)
    metdata = d.pop(0)
    pr = 0
    if child != 0:
        for i in range(child):
            answer = tree_creater(d, answer)
    elif metdata != 0:
        for i in range(metdata):
            answer += d.pop(0)
            pr = 1
    else:
        for i in range(metdata):
            answer += d.pop(0)
        pr = 1
    if pr != 1:
        for i in range(metdata):
            answer += d.pop(0)

    return answer


if __name__ == "__main__":
    sys.setrecursionlimit(15000)
    print(tree_creater(find_items(), 0))

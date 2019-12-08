import re


class Point_tree(object):
    def __init__(self, point, list_point_before, list_point_after):
        self.point = point
        self.list_point_before = sorted(list_point_before)
        self.list_point_after = sorted(list_point_after)

    def start_point(self):
        if not self.list_point_before:
            return True
        else:
            return False


def point_list(lis):
    _list_p = []
    for i in range(len(lis)):
        for j in range(len(lis[i])):
            if lis[i][j] not in _list_p:
                _list_p.append(lis[i][j])
    return _list_p


def list_ret(answer, list_start):
    for i in list_class_point:
        if i not in list(answer):
            if i.point == point_now:
                list_start += i.list_point_after
                return list_start


if __name__ == "__main__":
    d = []
    all_point = []
    answer = ''
    list_class_point = []
    with open("num.txt") as file_handler:
        for line in file_handler:
            d.append([line[5], line[36]])
    all_point = point_list(d)
    list_point_before = []
    list_point_after = []
    list_start = list()
    list_midlle = []
    for i in all_point:
        for j in d:
            if i == j[0]:
                list_point_after.append(j[1])
            elif i == j[1]:
                list_point_before.append(j[0])
        list_class_point.append(Point_tree(i, list_point_before, list_point_after))
        list_point_before, list_point_after = [], []

    for i in list_class_point:
        if i.start_point():
            list_start.append(i.point)
    all_point = [x for x in all_point if x not in list_start]
    print(all_point)
    print(list_start)
    kol=len(list_start)
    point_now = ''
    point_now2 = ''
    while True:
        list_start = set(list_start)
        list_start = list(list_start)
        list_start = sorted(list_start)
        point_now = list_start.pop(0)
        list_for_answer = list(answer)
        for i in list_class_point:
            if i.point == point_now:
                if i.list_point_before not in list(answer):
                    if list_start:
                        list_start = sorted(list_start)
                        point_now2 = list_start.pop(0)
                        list_start.insert(0, point_now)
                        point_now = point_now2
        if point_now not in answer:
            answer += point_now
        if list_start:
            list_start = list_ret(answer, list_start)
        if len(answer) >= len(all_point)+kol:
            break

print(answer)

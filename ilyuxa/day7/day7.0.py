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


def list_ret(point, list_start):
    for i in list_class_point:
        if i.point == point:
            list_start += i.list_point_after
    return list_start


if __name__ == "__main__":
    d, all_point, answer, list_class_point = [], [], '', []
    with open("num.txt") as file_handler:
        for line in file_handler:
            d.append([line[5], line[36]])
    list_point_before, list_point_after, all_point, list_start = [], [], point_list(d), list()
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
    kol, point_now, point_now2 = len(list_start), '', ''
    while True:
        list_start = sorted(list(set(list_start)))
        point_now = list_start.pop(0)
        for i in list_class_point:
            if point_now == i.point:
                if answer != []:
                    if i.list_point_before != []:
                        if set(i.list_point_before).issubset(answer):
                            list_start = list_ret(point_now, list_start)
                            answer += point_now
                            list_start += point_now2
                            point_now2 = ''
                            break
                        else:
                            point_now2 += point_now
                            break
                    else:
                        answer += point_now
                        list_start = list_ret(point_now, list_start)
                        break
                else:
                    answer += point_now
                    list_start = list_ret(point_now, list_start)
                    break
        if len(answer) == kol + len(all_point):
            break
print(answer)

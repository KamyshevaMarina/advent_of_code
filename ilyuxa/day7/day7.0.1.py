#https://adventofcode.com/2018/day/7
import re


class Point_tree():
    def __init__(self, point, list_point_before, list_point_after):
        self.point = point
        self.list_point_before = sorted(list_point_before)
        self.list_point_after = sorted(list_point_after)


def return_clas_list(list_point, a_before_b_list):  # возвращает списко объектов класа для каждой точки
    list_point_before, list_point_after, list_class_point = [], [], {}
    for i in list_point:
        for j in a_before_b_list:
            if i == j[0]:
                list_point_after.append(j[1])
            elif i == j[1]:
                list_point_before.append(j[0])
        list_class_point[i] = (Point_tree(i, list_point_before, list_point_after))
        list_point_before, list_point_after = [], []
    return list_class_point


def list_st(listst):
    listst = list(set(listst))
    listst = sorted(listst)
    return listst


def main():
    d, allpoint, list_class_point, point_now, answer, list_statement, unclaimed = [], [], {}, '', '', [], []
    with open("num.txt") as file_handler:
        for line in file_handler:
            d.append([line[5], line[36]])
            allpoint.append(line[5])
            allpoint.append(line[36])
    allpoint = set(allpoint)
    list_class_point = return_clas_list(allpoint, d)
    for i in list_class_point.items():
        if i[1].list_point_before == []:
            list_statement.append(i[0])
    while True:
        list_statement = list_st(list_statement)
        point_now = list_statement.pop(0)
        if set(list_class_point[point_now].list_point_before).issubset(answer) or answer == '':
            answer += point_now
            list_statement = list_statement + list_class_point[point_now].list_point_after + unclaimed
            unclaimed = []
        else:
            unclaimed.append(point_now)
        if len(answer) >= len(allpoint):
            break
    print(answer)
    print('EUGJKYFQSCLTWXNIZMAPVORDBH')


if __name__ == "__main__":
    main()

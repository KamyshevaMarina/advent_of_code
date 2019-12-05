import re


def manhattan_distance(v1, v2) -> int:
    return sum(abs(int(x) - int(y)) for (x, y) in zip(v1, v2))


def field_boundary(a):
    for i in range(len(a)):
        for j in range(len(a[i])):
            if i == 0:
                _minLeftx, _minLefty, _maxrightx, _maxrighty = a[i][j], a[i][j], a[i][j], a[i][j]
            else:
                if j == 0:
                    if int(a[i][j]) > int(_maxrightx):
                        _maxrightx = a[i][j]
                    elif int(a[i][j]) < int(_minLeftx):
                        _minLefty = a[i][j]
                if j == 1:
                    if int(a[i][j]) > int(_maxrighty):
                        _maxrighty = a[i][j]
                    elif int(a[i][j]) < int(_minLefty):
                        _minLeftx = a[i][j]
    return [(_minLeftx, _minLefty), (_maxrightx, _maxrighty)]


def midle_point(a):
    __ref = field_boundary(a)
    answer = []
    for i in a:
        if i[0] != __ref[0][0] and i[0] != __ref[1][0] and i[1] != __ref[0][1] and i[1] != __ref[1][1]:
            answer.append(i)
    return answer


def max_value(a={}):
    _max_value = 0
    for j in a.values():
        if j > _max_value:
            _max_value = j
    return _max_value


def find_Largest_Area():
    d = []
    with open("num.txt") as file_handler:  # в файле всегда в конце строки должен быть символ \n
        for line in file_handler:
            d.append(tuple([x for x in re.split(r'\D', line) if
                            x]))  # TODO принимаем входные значения и удаяляя все лишнее делаем список со списками координат
    field_boundary1 = field_boundary(d)
    print(field_boundary1)
    dict_point = {i: 0 for i in d}
    dist = 0
    keyD = tuple()
    gate = 0
    manhattan_dista = 0
    for i in range(int(field_boundary1[0][0])-1, int(field_boundary1[1][0]) + 2):
        for j in range(int(field_boundary1[0][1])-1, int(field_boundary1[1][1]) + 2):
            for key in dict_point:
                manhattan_dista = abs(manhattan_distance(key, [i, j]))
                dist += manhattan_dista
            if dist < 10000:
                gate += 1
            dist = 0
            #  print('Ключ ' + str(keyD) + 'Точка ' + '(' + str(i) + ',' + str(j) + ')' + 'дистанция ' + str(dist))
    print(gate)


if __name__ == "__main__":
    find_Largest_Area()

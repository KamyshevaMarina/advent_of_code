c = []  # где все храним
answer = ''  # ответ наш
times = 0  # сколько ошибок
endA = 0  # завершитель
indZ = 0
with open("num.txt") as file_handler:
    for line in file_handler:
        c.append(list(line[:-1]))
for g in range(0, (len(c))):  # бегаем по каждому списку в списке
    for j in range(0, (len(c))):  # по спискам которым сравниваем
        if c[g] == c[j]:
            continue
        for i in range(0, len(c[j])):  # по элементам
            if c[g][i] == c[j][i]:  # проверка на сходство
                answer += c[g][i]
            else:
                times += 1  # если ошибся считаем сколько раз
                if times >= 2:  # если два раз выходим нахрен
                    break
            if times == 1 and i==(len(c[j])-1):
                print(answer)
                endA = 1
                break
        if times >= 2:
            times = 0
            # print(answer)
            answer = ''
            continue
        elif endA == 1:
            break
    if endA == 1:
        break

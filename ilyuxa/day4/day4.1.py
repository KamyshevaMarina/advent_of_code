from dateutil import parser
import operator
import re

d = []
dTime = []
srtDoc = []
with open("num.txt") as file_handler:  # в файле всегда в конце строки должен быть символ \n
    for line in file_handler:
        d.append(line[:-1])
        dTime.append(line[1:17])

dTime = sorted(dTime, key=lambda x: parser.parse(x))
for i in dTime:
    for j in d:
        if i in j:
            srtDoc.append(j)
# тут заканчивается код сортировки по датам
print(srtDoc)
listIdandTime = []
gate = -1


def lineId(a):
    a = re.split(r'\D', a)
    a = [x for x in a if x]
    return a[-1:][0]


for line in srtDoc:
    if '#' in line:
        gate += 1
        listIdandTime.append([])
        listIdandTime[gate].append(lineId(line))
    elif 'll' in line:
        listIdandTime[gate].append('-' + line[15:17])
    elif 'w' in line:
        listIdandTime[gate].append((int((line[15:17]))))
print(listIdandTime)
# список по дням айцдли охраника и есго минуты пробуждения и сна
idGuard = set()
for i in listIdandTime:
    idGuard.add(i[0])
print(idGuard)
# общий список охраны
listGuard = []
gate2 = 0
for i in idGuard:
    listGuard.append([i])

    gate2 += 1
print('переменная lisguard' + str(listGuard))
# охрианики и общее время сна
max = 0
# for i in range(len(listGuard)):
#     if listGuard[i][1] > max:
#         max = listGuard[i][1]
#         maxsleeper = listGuard[i][0]
# print('дольше все спит охраник ' + maxsleeper)
# выбор охраника который дольше всего спит завершен
listGuard=list(listGuard)
minutesOfmaxsleeper = []
swap = 0
for j in listGuard:
    minutesOfmaxsleeper.append([j[0]])
    for i in range(len(listIdandTime)):
        if listIdandTime[i][0] == j[0]:
            minutesOfmaxsleeper[swap] = minutesOfmaxsleeper[swap] + listIdandTime[i][1:]
    swap += 1
print('охраники плюс их все время' + str(minutesOfmaxsleeper))
# все таймзорны сна охраника
dict1 = {i: 0 for i in range(0, 60)}
maxValue = 0
maxValue2 = 0
minute = 0
minute2 = 0
guard2 = ''
for l in range(len(minutesOfmaxsleeper)):
    for i in range(0, 59):
        for j in range(1, len(minutesOfmaxsleeper[l]), 2):
            for k in range(abs(int(minutesOfmaxsleeper[l][j])), minutesOfmaxsleeper[l][j + 1]):
                if i == k:
                    dict1[i] += 1
    for m, f in dict1.items():
        if f > maxValue:
            maxValue = f
            minute = m
    if maxValue > maxValue2:
        maxValue2 = maxValue
        minute2 = minute
        guard2 = minutesOfmaxsleeper[l][0]
    dict1 = {i: 0 for i in range(0, 60)}

print(str(minute) + 'минута и сколько раз' + str(maxValue) + ' номер охраника' + str(guard2))
answer = minute * int(guard2)
print('Ответ' + str(answer))

# final_dict = dict([max(dict1.items(), key=lambda k_v: k_v[1])])
# print('мксимальное значени'+final_dict)

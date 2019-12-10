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
        print(lineId(line))
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
    listGuard[gate2].append(0)
    for j in listIdandTime:
        if j[0] == i:
            for item in j:
                if j[0] != item:
                    listGuard[gate2][1] += int(item)
    gate2 += 1
print(listGuard)
# охрианик и общее время сна
maxsleeper = 0
max = 0
for i in range(len(listGuard)):
    if listGuard[i][1] > max:
        max = listGuard[i][1]
        maxsleeper = listGuard[i][0]
print('дольше все спит охраник ' + maxsleeper)
# выбор охраника который дольше всего спит завершен
minutesOfmaxsleeper = []
for i in range(len(listIdandTime)):
    if listIdandTime[i][0] == maxsleeper:
        minutesOfmaxsleeper = minutesOfmaxsleeper + listIdandTime[i][1:]
print(minutesOfmaxsleeper)
# все таймзорны сна охраника
dict1 = {i: 0 for i in range(0, 60)}

for i in range(0, 59):
    for j in range(0, len(minutesOfmaxsleeper), 2):
        for k in range(abs(int(minutesOfmaxsleeper[j])), minutesOfmaxsleeper[j + 1]):
            if i == k:
                dict1[i] += 1
maxValue=0
minute=0
for i,j in dict1.items():
    if j>maxValue:
        maxValue=j
        minute=i
print(str(minute)+'минута и сколько раз'+str(maxValue))
answer = minute*int(maxsleeper)
print('Ответ'+str(answer))

# final_dict = dict([max(dict1.items(), key=lambda k_v: k_v[1])])
# print('мксимальное значени'+final_dict)

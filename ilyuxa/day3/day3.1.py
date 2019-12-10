import re
from time import time

tic = time()
answer = 0
c = [0] * 1000  # одномерный
d = []
for i in range(len(c)):
    c[i] = [0] * 1000  # двухмерный
with open("num.txt") as file_handler:  # в файле всегда в конце строки должен быть символ \n
    for line in file_handler:
        d.append(line[:-1])
for i in range(len(d)):
    d[i] = re.split(r'\D', d[i])
    d[i] = list(filter(None, d[i]))
for i in range(len(d)):
    for j in range((int(d[i][2])), (int(d[i][2]) + int(d[i][4]))):
        for k in range((int(d[i][1])), ((int(d[i][1])) + (int(d[i][3])))):
            if c[j][k] != 'X':
                c[j][k] += 1
                if c[j][k] >= 2:
                    c[j][k] = 'X'
            else:
                continue
gate = 0
for i in range(len(d)):
    for j in range((int(d[i][2])), (int(d[i][2]) + int(d[i][4]))):
        for k in range((int(d[i][1])), ((int(d[i][1])) + (int(d[i][3])))):
            if c[j][k] == 'X':
                gate = 1
                break
            if j == ((int(d[i][2]) + int(d[i][4]))-1) and k == (((int(d[i][1])) + (int(d[i][3])))-1):
                print(d[i][0])
                answer = d[i][0]
                gate = 2
                break
        if gate == 1 or gate == 2:
            break
    if gate == 1:
        gate = 0
        continue
    elif gate == 2:
        break
toc = time()
print(toc - tic)
print(answer)
handle = open("output.txt", "w")
# for i in c:
#     dl = ''.join(str(i))
#     handle.write(str(dl) + "\n")
# handle.close()

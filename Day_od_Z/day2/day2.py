from collections import Counter

twoTimes = 0
threetimes = 0
c = []
answer = 0
with open("num.txt") as file_handler:
    for line in file_handler:
        c = Counter(list(line))
        dict(c)
        if 2 in c.values():
            twoTimes += 1
        if 3 in c.values():
            threetimes += 1
        c = []
answer = int(twoTimes) * int(threetimes)
print('ОТвет:' + str(answer))

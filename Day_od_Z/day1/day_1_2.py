import collections

answer = 0
a = set()
found = 0
iter1 = 0
while True:
    with open("num.txt") as file_handler:
        for line in file_handler:
            answer += int(line)
            if int(answer) in a:
                found = 1
                break
            a.add(int(answer))
        if found == 1:
            break
    iter1 += 1
    if iter1 == 1000000:
        print(a)
        break
print("Проходов: " + str(iter1))
print("Ответ: " + str(answer))

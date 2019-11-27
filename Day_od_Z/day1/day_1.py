answer = 0
with open("num") as file_handler:
    for line in file_handler:
        answer += float(line)
print(answer)

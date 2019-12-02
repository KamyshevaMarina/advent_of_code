d = ''
with open("num.txt") as file_handler:  # в файле всегда в конце строки должен быть символ \n
    for line in file_handler:
        d += line
c = [None]
print(d)


def AaBbCc(A, B) -> bool:
    if B is None:
        return True
    if A.upper() == B.upper():
        if ('a' <= A <= 'z' and 'A' <= B <= 'Z') or ('a' <= B <= 'z' and 'A' <= A <= 'Z'):
            return False
        else:
            return True
    else:
        return True


n = bool
for i in d:
    if AaBbCc(i, c[-1]):
        c.append(i)
    else:
        c = c[:-1]
s_out = ''.join(c[1:])
print(len(s_out))
handle = open("output.txt", "w")
handle.write(s_out)
# handle.close()

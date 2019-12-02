d = ''
with open("num.txt") as file_handler:  # в файле всегда в конце строки должен быть символ \n
    for line in file_handler:
        d += line
c = [None]


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


striiing = 'abcdefghijklmnopqrstuvwxyz'
n = bool
Oldc = ''
min = 9999999999999999
for l in striiing:
    for j in d:
        if j != l and j != l.upper():
            Oldc += j
    for i in Oldc:
        if AaBbCc(i, c[-1]):
            c.append(i)
        else:
            c = c[:-1]
    s_out = ''.join(c[1:])
    if len(s_out) < min:
        min = len(s_out)
    s_out = ''
    Oldc = ''
    c = [None]

print(min)

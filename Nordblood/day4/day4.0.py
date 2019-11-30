import arrow
from dateutil import parser
import re

d = []
doc=[]

with open("num.txt") as file_handler:  # в файле всегда в конце строки должен быть символ \n
    for line in file_handler:
        doc.append(line)
for line in doc:
    d.append(line[1:17])
d = sorted(d, key=lambda x: parser.parse(x))
print(d)

for i in d:
    for j in doc:
        if j[1:16] == i[0:15]:
            print(j)
i = [int(l.strip('\n')) for l in open("input/1.txt", "r")]

for x in i:
    for y in i:
        if x + y == 2020:
            print(x*y)


for x in i:
    for y in i:
        for z in i:
            if x + y + z == 2020:
                print(x*y*z)

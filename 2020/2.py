import re
lines = open("input/2.txt", 'r').readlines()

#regex for: 3-11 z: zzzzzdzzzzlzz
p = re.compile('(\d+)-(\d+)\s([a-z]):\s([a-z]+)')
part1_valid_pws = []
part2_valid_pws = []

for l in lines:
    (lower, upper, c, pw) = p.match(l).groups()
    lower = int(lower)
    upper = int(upper)
    count = pw.count(c)
    if count <= upper and count >= lower:
        part1_valid_pws.append(pw)
    if (pw[lower-1] == c or pw[upper-1] == c) ^ (pw[lower-1] == c and pw[upper-1] == c):
        part2_valid_pws.append(pw)

print("Part 1: ", len(part1_valid_pws))
print("Part 2: ", len(part2_valid_pws))
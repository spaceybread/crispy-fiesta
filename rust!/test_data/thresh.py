def dist(a, b):
    s = 0
    if len(a) != len(b):
        print(len(a), len(b), "mismatch")
        print(b)
        return 0
    for i in range(len(a)):
        s += a[i]**2 + b[i]**2
    return s**0.5

out = []
for i in range(1, 10178):

    filepath = "imgs/" +  str(i) + ".txt"
    f = open(filepath, 'r')

    vals = []

    for line in f:
        vals.append(list(map(float, line.split(", "))))
    
    print(i, len(vals))
    
    m = 0
    for i in range(len(vals)):
        for j in range(i + 1, len(vals)):
            if (dist(vals[i], vals[j]) > m):
                m = dist(vals[i], vals[j])
    out.append(m)

out.sort()

print(out[0], out[-5], out[-4], out[-3], out[-2], out[-1])

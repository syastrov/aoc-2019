from math import floor

with open("1.txt") as f:
  lines = f.readlines()

def f(v):
  if v > 0:
    return f(floor(v / 3) - 2) + max(floor(v / 3) - 2, 0)
  return 0


s = 0
for line in lines:
  s += f(int(line))
  
print(s)
  

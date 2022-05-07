# Solve for x*DATA_POINTS = TO_WRITE to figure out what exponential step
# will give us approximately DATA_POINTS worth of data for the x axis of our
# graph
TO_WRITE = 128 * 1024 * 1024
DATA_POINTS = 512

scale = TO_WRITE ** (1 / DATA_POINTS)

x = 4
while x < TO_WRITE:
    print(x, end='usize,')

    # Advance at _least_ 4 bytes
    x = max(x + 4, int(float(x) * scale))

    # 4-byte align value
    x = x & ~3

print()


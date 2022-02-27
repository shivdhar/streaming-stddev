import math
from numbers import Number


class Welford:
    def __init__(self):
        self.n = 0
        self.M = 0
        self.S = 0

    def add(self, num: Number) -> None:
        num = float(num)

        self.n += 1

        num_diff = num - self.M
        M2 = self.M + (num_diff / self.n)
        S2 = self.S + num_diff * (num - M2)

        self.M = M2
        self.S = S2

    @property
    def stddev(self) -> float:
        n = self.n
        if n == 1:
            return 0.
        stddev = math.sqrt(self.S / n)
        return stddev

from array import array
from statistics import pstdev
import pytest

def test_welford():
    from welford import Welford
    # arr = [1, 2, 3]
    arr = range(1000)
    stdlib_val = pstdev(arr)
    welford = Welford()
    for num in arr:
        welford.add(num)
    assert welford.stddev == pytest.approx(stdlib_val)

# test_welford()
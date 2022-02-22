import numpy as np
import statistics


def stddev(arr):
    mean = arr.mean()

    var = (arr - mean) ** 2
    
    var = var.sum() / len(var)

    stdev = np.sqrt(var)
    return stdev


def main():
    a = np.arange(10)

    print(a)
    print(stddev(a))

    b = statistics.pstdev(a)
    print(b)

    c = np.std(a)
    print(c)


if __name__ == '__main__':
    main()

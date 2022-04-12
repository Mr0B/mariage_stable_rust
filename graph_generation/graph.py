import os


import matplotlib.pyplot as plt


def extract_value():
    path = os.path.abspath("graph_generation/log.txt")
    file_object = open(path, "r")
    i = 0
    list1 = []
    storage = (0, 0)
    for line in file_object:
        print(storage)
        list2 = line.split("/")
        if i % 2 == 0:
            size = int(list2[0])
            time = int(list2[2])
            storage = (size, time)
        else:
            time = int(list2[2])
            list1.append((storage[0], time / storage[1]))
        i += 1
    return list1


def basic_plot(list1):
    plt.close('all')
    x = [i[0] for i in list1]
    print(x)
    y = [i[1] for i in list1]
    print(y)
    plt.grid()
    plt.plot(x, y)
    plt.show()


if __name__ == '__main__':
    list_size_speedup = extract_value()
    basic_plot(list_size_speedup)

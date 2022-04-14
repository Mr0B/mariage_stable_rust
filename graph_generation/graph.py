import datetime
import os
import subprocess

import matplotlib.pyplot as plt


def extract_value():
    path = os.path.abspath("graph_generation/log.txt")
    file_object = open(path, "r")
    i = 0
    list1 = []
    storage = (0, 0)
    for line in file_object:
        i += 1
        list2 = line.split("/")
        if i % 2 == 0:
            time = int(list2[2])
            list1.append((storage[0], storage[1] / time))
        else:
            size = int(list2[0])
            time = int(list2[2])
            storage = (size, time)
    file_object.close()
    return list1


def basic_plot(list1, title):
    plt.close('all')
    x = [i[0] for i in list1]
    y = [i[1] for i in list1]
    plt.ylabel("Speed_Up")  # y label
    plt.xlabel("Size_Instance")  # x label
    plt.grid()
    plt.plot(x, y)
    plt.title(title)
    plt.savefig(f'graph_generation/{datetime.datetime.now()}.png')


if __name__ == '__main__':
    command = 'cargo run -- --instance-size-start 100 --instance-size-end 1000 -p 25 -s 42'
    subprocess.run(command, shell=True)
    list_size_speedup = extract_value()
    basic_plot(list_size_speedup, command)

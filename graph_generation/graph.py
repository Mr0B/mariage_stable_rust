import datetime
import os
import subprocess

import matplotlib.pyplot as plt
import numpy as np


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


def parallel_plot():
    path = os.path.abspath("graph_generation/10to1000by10with101try.txt")
    file_object = open(path, "r")
    list1 = []
    for line in file_object:
        list2 = line.split('/')
        if list2[1] != 'Sequential':
            list1.append((int(list2[0]), int(list2[2])))
    list3 = median(list1)
    file_object.close()
    plt.close('all')
    x = [i[0] for i in list3]
    y = [i[1] for i in list3]
    plt.ylabel("Time")  # y label
    plt.xlabel("Size_Instance")  # x label
    plt.grid()
    plt.plot(x, y)


def sequential_plot():
    path = os.path.abspath("graph_generation/10to1000by10with101try.txt")
    file_object = open(path, "r")
    list1 = []
    for line in file_object:
        list2 = line.split('/')
        if list2[1] == 'Sequential':
            list1.append((int(list2[0]), int(list2[2])))
    list3 = median(list1)
    # plt.close('all')
    x = [i[0] for i in list3]
    y = [i[1] for i in list3]
    plt.ylabel("Time")  # y label
    plt.xlabel("Size_Instance")  # x label
    plt.grid()
    plt.plot(x, y)
    # plt.show()
    plt.savefig(f'graph_generation/Graphs/{datetime.datetime.now()}.png')


def moyenne(list_tuple):
    list_moyenne = []
    holder = []
    for x in range(0, len(list_tuple)):
        if x != len(list_tuple) - 1 and list_tuple[x][0] == list_tuple[x + 1][0]:
            holder.append(list_tuple[x][1])
        else:
            holder.append(list_tuple[x][1])
            list_moyenne.append((list_tuple[x][0], sum(holder) / len(holder)))
            holder.clear()
    return list_moyenne


def median(list_tuple):
    list_median = []
    holder = []
    for x in range(0, len(list_tuple)):
        if x != len(list_tuple) - 1 and list_tuple[x][0] == list_tuple[x + 1][0]:
            holder.append(list_tuple[x][1])
        else:
            holder.append(list_tuple[x][1])
            list_median.append((list_tuple[x][0], np.median(holder)))
            holder.clear()
    return list_median


def basic_plot(list1, title):
    plt.close('all')
    x = [i[0] for i in list1]
    y = [i[1] for i in list1]
    plt.ylabel("Speed_Up")  # y label
    plt.xlabel("Size_Instance")  # x label
    plt.grid()
    plt.plot(x, y)
    plt.title(title)
    plt.savefig(f'graph_generation/Graphs/{datetime.datetime.now()}.png')


if __name__ == '__main__':
    command = 'cargo run -- --instance-size-start 100 --instance-size-end 500 -p 20 -t 4 -n 11'
    subprocess.run(command, shell=True)
    list_size_speedup = extract_value()
    better_list = median(list_size_speedup)
    basic_plot(better_list, command)
    # parallel_plot()
    # sequential_plot()
    # subprocess.run('rm graph_generation/log.txt', shell=True)

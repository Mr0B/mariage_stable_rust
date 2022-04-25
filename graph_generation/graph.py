import datetime
import os
# import subprocess
from collections import namedtuple

import matplotlib.pyplot as plt
import numpy as np


def extract_value_1(string):
    path = os.path.abspath(string)
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


Data = namedtuple("Data", ["size", "time_seq", "time_par"])


def extract_value_2(path):
    path = os.path.abspath(path)
    with open(path, "r") as f:
        return [parse_line(line1, line2) for (line1, line2) in zip(f, f)]


def parse_line(line1, line2):
    [size, _, time_seq] = line1.split("/")
    [_, _, time_par] = line2.split("/")
    return Data(int(size), int(time_seq), int(time_par))


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


def sequential_time_100():
    path = os.path.abspath("graph_generation/10to1000by10with101try.txt")
    file_object = open(path, "r")
    list_holder = []
    for line in file_object:
        list2 = line.split('/')
        if list2[1] == 'Sequential':
            list_holder.append((int(list2[0]), int(list2[2])))
    list_moyenne = []
    holder = []
    for x in range(0, len(list_holder)):
        if x != len(list_holder) - 1 and (list_holder[x][0] // 100) == (list_holder[x + 1][0] // 100):
            holder.append(list_holder[x][1])
        else:
            holder.append(list_holder[x][1])
            list_moyenne.append((list_holder[x][0], sum(holder) / len(holder)))
            holder.clear()
    return list_moyenne


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
    plt.legend(["parallèle", "séquentiel"], loc="lower right")
    # plt.show()
    plt.title("Temps d'exécution/Taille de l'instance 10-1000 pas=10")
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


def group_data(data_list):
    grouped_data = {}
    for data in data_list:
        if data.size not in grouped_data:
            grouped_data[data.size] = []
        grouped_data[data.size].append(data)
    return sorted(grouped_data.values(), key=lambda l: l[0].size)


def basic_plot(list1):
    plt.close('all')
    x = [i[0] for i in list1]
    y = [i[1] for i in list1]
    plt.ylabel("Speed_Up")  # y label
    plt.xlabel("Size_Instance")  # x label
    plt.grid()
    plt.plot(x,y)
    plt.title("Speed_up/Taille Instance 10-1000 pas=25/Worst-Case")
    plt.savefig(f'graph_generation/Graphs/{datetime.datetime.now()}.png')


def threads_iterations_plots(list1, list2, list3):
    plt.close('all')
    a = [i[0] for i in list1]
    b = [i[1] for i in list1]
    c = [i[0] for i in list2]
    d = [i[1] for i in list2]
    e = [i[0] for i in list3]
    f = [i[1] for i in list3]
    plt.ylabel("Speed_Up")  # y label
    plt.xlabel("Size_Instance")  # x label
    plt.grid()
    plt.plot(a, b)
    plt.plot(c, d)
    plt.plot(e, f)
    plt.legend(["2 thread", "3 thread", "4 thread"], loc="lower right")
    plt.title("Speed_up/Taille Instance 100-200(pas=10)/nombre_threads")
    plt.savefig(f'graph_generation/Graphs/{datetime.datetime.now()}.png')


if __name__ == '__main__':
    command = 'cargo run -- --instance-size-start 100 --instance-size-end 1000 -p 25 -t 4 -n 51 -w'
    # subprocess.run(command, shell=True)
    # list_speedup_2 = moyenne(extract_value("graph_generation/seed42threads2.txt"))
    # list_speedup_3 = moyenne(extract_value("graph_generation/seed42threads3.txt"))
    # list_speedup_4 = moyenne(extract_value("graph_generation/seed42threads4.txt"))
    # threads_iterations_plots(list_speedup_2, list_speedup_3, list_speedup_4)
    list_size_speedup = extract_value_1("graph_generation/100to1000by25with51tryWorstCase.txt")
    better_list = moyenne(list_size_speedup)
    basic_plot(better_list)
    # parallel_plot()
    # sequential_plot()
    # subprocess.run('rm graph_generation/log.txt', shell=True)
    # print(sequential_time_100())

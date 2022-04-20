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


def moyenne(list_tuple):
    list_moyenne = []
    holder = []
    for x in range(0, len(list_tuple)):
        if x != len(list_tuple)-1 and list_tuple[x][0] == list_tuple[x + 1][0]:
            holder.append(list_tuple[x][1])
        else:
            holder.append(list_tuple[x][1])
            list_moyenne.append((list_tuple[x][0], sum(holder) / len(holder)))
            holder.clear()
    return list_moyenne


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
    command = 'cargo run -- -s 42 --instance-size-start 100 --instance-size-end 1000 -p 25 -t 4 -n 10'
    subprocess.run(command, shell=True)
    list_size_speedup = extract_value()
    better_list = moyenne(list_size_speedup)
    basic_plot(better_list, command)

#!/usr/bin/python3
# -*- coding: utf-8 -*-

from collections import namedtuple
from queue import Queue, LifoQueue
Item = namedtuple("Item", ['index', 'value', 'weight'])



def solve_it(input_data):
    # Modify this code to run your optimization algorithm

    # parse the input
    lines = input_data.split('\n')

    firstLine = lines[0].split()
    item_count = int(firstLine[0])
    capacity = int(firstLine[1])

    items = []

    for i in range(1, item_count+1):
        line = lines[i]
        parts = line.split()
        items.append(Item(i-1, int(parts[0]), int(parts[1])))

    optimal = 1
    if item_count < 400 or len(items) == 1000:
        value, taken = dynamic_programming(items, capacity)
    else:
        optimal = 0
        value, taken = greedy(items, capacity)

    output_data = str(value) + ' ' + str(optimal) + '\n'
    output_data += ' '.join(map(str, taken))
    return output_data


def dynamic_programming(items, capacity):
    item_count = len(items)

    taken = [0]*len(items)
    cache = [[0 for i in range(capacity + 1)] for j in range(item_count + 1)]
    
    for i in range(item_count + 1):
        for j in range(capacity + 1):
            if i == 0 or j == 0:
                cache[i][j] = 0
            elif items[i - 1].weight <= j:
                cache[i][j] = max(items[i - 1].value + cache[i - 1][j - items[i - 1].weight], cache[i - 1][j])
            else:
                cache[i][j] = cache[i - 1][j]

    value = cache[item_count][capacity]

    column = capacity
    row = item_count
    while column > 0 and row > 0:
        if cache[row][column] == cache[row - 1][column]:
            row -= 1
        else:
            taken[row - 1] = 1
            column -= items[row - 1].weight
    
    return value, taken


def branch_and_bound(items, capacity):
    sorted_items = sorted(items, key=lambda item: -item.value)
    items = sorted(sorted_items, key=lambda item: -item.value/item.weight)

    def calculate_max_profit(cur_value, capacity, index):
        # print(cur_value, capacity)
        cur_items = items[index:]
        # print(cur_items)

        remaining_capacity = capacity
        i = 0
        for item in cur_items:
            # print(item, remaining_capacity, cur_value)
            if item.weight <= remaining_capacity:
                i += 1
                cur_value += item.value
                remaining_capacity -= item.weight
            else:
                break
        
        if (i + index) != len(items):
            # print('<<<<<<<<', item, cur_value, remaining_capacity)
            # print(cur_value, remaining_capacity, item)
            cur_value += item.value * remaining_capacity / float(item.weight)

        # print(cur_value, remaining_capacity)
        return cur_value
        
    max_value = 0

    queue = LifoQueue()
    queue.put((0, capacity, 0))
    
    while not queue.empty():
        cur_value, cur_capacity, index = queue.get()

        if cur_capacity < 0:
            continue
        
        if cur_value > max_value:
            max_value = cur_value
        
        if cur_capacity <= 0 or index >= len(items):
            continue
        
        # print('>>>', cur_value, cur_capacity)
        cur_max_profit = calculate_max_profit(cur_value, cur_capacity, index)
        # print(cur_value, cur_capacity, index, cur_max_profit, max_value)
        # print(cur_max_profit)

        if cur_max_profit < max_value:
            continue

        queue.put((cur_value, cur_capacity, index + 1))
        queue.put((cur_value + items[index].value, cur_capacity - items[index].weight, index + 1))

    return max_value, []


def greedy(items, capacity):
    sorted_items = sorted(items, key=lambda item: -item.value)
    items = sorted(sorted_items, key=lambda item: -item.value/item.weight)

    value = 0
    taken = [0]*len(items)
    remaining_capacity = capacity
    for item in items:
        if item.weight < remaining_capacity:
            taken[item.index] = 1
            value += item.value
            remaining_capacity -= item.weight

    return value, taken


if __name__ == '__main__':
    import sys
    if len(sys.argv) > 1:
        file_location = sys.argv[1].strip()
        with open(file_location, 'r') as input_data_file:
            input_data = input_data_file.read()
        print(solve_it(input_data))
    else:
        print('This test requires an input file.  Please select one from the data directory. (i.e. python solver.py ./data/ks_4_0)')


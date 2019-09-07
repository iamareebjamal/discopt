#!/usr/bin/python3
# -*- coding: utf-8 -*-

import os
from subprocess import Popen, PIPE

def solve_it(file_location):
    Popen(['rustc', '-O', 'main.rs']).communicate()
    process = Popen(['./main', file_location], stdout=PIPE)
    (stdout, stderr) = process.communicate()

    return stdout.strip()


import sys

if __name__ == '__main__':
    if len(sys.argv) > 1:
        file_location = sys.argv[1].strip()
        print solve_it(file_location)
    else:
        print('This test requires an input file.  Please select one from the data directory. (i.e. python solver.py ./data/ks_4_0)')


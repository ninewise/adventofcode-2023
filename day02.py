#!/usr/bin/env python
import sys

def games(filename):
	with open(filename) as f:
		for line in f:
			gameids, subsetss = line.strip().split(": ")
			gameid = int(gameids[5:])
			subsets = []
			for subset in subsetss.split("; "):
				current = {'red': 0, 'green': 0, 'blue': 0}
				subsets.append(current)
				for ncolor in subset.split(", "):
					n, color = ncolor.split(" ")
					current[color] = int(n)
			yield gameid, subsets

def p1(filename):
	red_max, green_max, blue_max = 12, 13, 14
	print(sum(gameid
			for gameid, subsets in games(filename)
			if all(subset['red'] <= red_max and
			       subset['green'] <= green_max and
			       subset['blue'] <= blue_max
			       for subset in subsets)))

def power(subsets):
	return max(s['red'] for s in subsets) * max(s['green'] for s in subsets) * max(s['blue'] for s in subsets)

def p2(filename):
	print(sum(power(subsets) for _, subsets in games(filename)))

if sys.argv[2] == "1":
	p1(sys.argv[3])
else:
	p2(sys.argv[3])

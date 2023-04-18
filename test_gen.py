# flags 
# -t test name
# -n number of nodes to add
# -e number of edges to add
# -r number of nodes to remove
# -m number of edges to remove

# TODO: add support for edge weight changes, etc

import argparse
parser = argparse.ArgumentParser()
parser.add_argument('--n', '--nc')
parser.add_argument('--e', '--ec')
parser.add_argument('--r', '--nr')
parser.add_argument('--m', '--er')
parser.add_argument('--t', '--test_name')

args = parser.parse_args()


# should this produce rust code, or should we take as input
# a file? probably the latter..


node_count = args.nc
edge_count = args.ec
node_removal_count = args.nr
edge_removal_count = args.er

# will basically just keep track of its own edge set and nodes..
# set of existing nodes
# set of 

# addnode id
# removenode id
node_num = 0


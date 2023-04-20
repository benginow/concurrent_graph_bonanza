import random

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
parser.add_argument('--u' '--undirected', action='store_true')
parser.add_argument('--t', '--test_name')

args = parser.parse_args()

node_count = args.nc
edge_count = args.ec
node_removal_count = args.nr
edge_removal_count = args.er

created_node_count = 0;
created_edge_count = 0;

# add node ids
existing_nodes = {}
# every time an edge is added, add the tuple this set
existing_edges = {}

# the file
f = open(args.test_name, "w")
f.write("")

def gen_add_node():
    str1 = "let val = g.add_node(" + str(node_count) + ");\n"
    str1 = str1 +  "assert!(val.is_ok());\n"
    str1 = "let val = g.add_node(" + str(node_count) + ");\n"
    str1 = str1 +  "assert!(val.is_err());\n"
    node_count = node_count - 1
    # now, write the string to the file
    f.write(str1)
    return

def gen_add_edge_addition(node_1, node_2):
    str1 = "let val = g.add_edge("+ str(node_1) + "," + str(node_2) + "," + str(random.random())  +");\n"
    f.write(str1)

def gen_add_edge_helper(node_1, node_2):
    gen_add_edge_addition(node_1, node_2)
    # check for existence
    exists = (node_1, node_2) in existing_edges
    # if it exists already, then you want to assert that the insertion did not complete
    if exists:
        str1 = str1 + "assert!(val == GraphErr::EdgeAlreadyExists)\n"
    else:
        str1 = str1 + "assert!(val.is_some())\n"
    f.write(str1)
    return

def gen_add_edge():
    # attempt to add the edge
    node_1 = random.choice(existing_nodes)
    node_2 = random.choice(existing_nodes)
    gen_add_edge_helper(node_1, node_2)
    if args.undirected:
        gen_add_edge_helper(node_2, node_1)

    gen_add_edge_addition(node_1, node_2)
    str1 = "assert!(val == GraphErr::EdgeAlreadyExists)\n"
    f.write(str1)

    edge_count = edge_count - 1

    return
    
# edge is a tuple
def gen_remove_edge_helper(edge):
    # to to add and assert that the edge exists
    # now, remove the edge and assert that it worked
    str1 = str1 + "let val = g.remove_edge(" + str(edge[0]) + "," + str(edge[1]) + ")\n"
    str1 = str1 + "assert!(val.is_ok())\n";
    f.write(str1)

def gen_remove_edge():
    # check that you can't add the node
    # then, remove it
    remove_me = random.choice(existing_edges)
    existing_edges.remove(remove_me)
    gen_remove_edge_helper(remove_me)

    if args.undirected:
        other_tuple = (remove_me[1], remove_me[0])
        existing_edges.remove(other_tuple)
        gen_remove_edge_helper(remove_me)

    edge_removal_count = edge_removal_count - 1

    # first, attempt 

def gen_remove_node():
    remove_me = random.choice(existing_nodes)
    existing_nodes.remove(remove_me)
    # could be good to go thru the whole graph and make sure that all of the 
    # connections are gone, but this would take way too long
    str1 = "let neighbors = g.get_neighbors( " + remove_me + ");\n"
    str1 = "let val = g.remove_node( " + remove_me + ");\nassert!(val.is_ok());\n";

    node_removal_count = node_removal_count - 1

still_in_progress = ["nc", "ec", "nrc", "erc"]
if (node_count > 0):
    still_in_progress.insert("nc")
if (edge_count > 0):
    still_in_progress.insert("ec")
if (node_removal_count > 0):
    still_in_progress.insert("nrc")
if (edge_removal_count > 0):
    still_in_progress.insert("erc")

num_nodes = 0
num_edges = 0
while (len(still_in_progress) > 0):
    curr = random.choose(still_in_progress)

    if (curr == "nc"):
        gen_add_node()
        num_nodes = num_nodes + 1
        if node_count == 0:
            still_in_progress.remove("nc")

    if (curr == "ec"):
        if (num_nodes >= 2):
            gen_add_edge()
            num_edges = num_edges + 1
        if (edge_count == 0):
            still_in_progress.remove("ec")

    if (curr == "nrc"):
        if num_nodes > 0:
            gen_remove_node()
            num_nodes = num_nodes - 1
        if (node_removal_count == 0):
            still_in_progress.remove("nrc")
    if (curr == "erc"):
        if num_edges > 0:
            gen_remove_edge()
            num_edges = num_edges - 1
        if (edge_removal_count == 0):
            still_in_progress.remove("erc")



# addnode id
# removenode id
node_num = 0


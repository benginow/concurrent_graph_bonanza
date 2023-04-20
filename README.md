# Testing
Stress testing


You may use the python script in order to generate a correctness test. Please do not generate very large tests since they are statically generated -- this is NOT a stress testing framework
n = how many nodes to add
e = how many edges to add
r = how many nodes to remove
m = how many edges to remove
GENERATE A TESTCASE:
python3 test_gen.py --t "src/test1.rs" -n10 -e20 -r0 -m0
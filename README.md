# Implementations
## Fine-grained locking of adjacency list
This implementation is under simple.rs

## Coarse-grained locking around CSR
This implementation is under coarse.rs

## Graphone
This implementation is under one.rs

# Testing
Stress testing
In tests.rs, you should find a bench function -- just run that in order to bench a stress test.


You may use the python script in order to generate a correctness test. Please do not generate very large tests since they are statically generated -- this is NOT a stress testing framework
n = how many nodes to add
e = how many edges to add
r = how many nodes to remove
m = how many edges to remove
GENERATE A TESTCASE:
python3 test_gen.py --t "src/test1.rs" -n10 -e20 -r0 -m0
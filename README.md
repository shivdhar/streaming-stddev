# Streaming calculation of standard deviation in Python and Rust

## Problem statement:

Create a program that calculates the standard deviation of a sequence of floating-point numbers. 
The program should incrementally update the stddev as new data comes in, i.e. not recompute over the entire sequence again and again.

## Proposed solution:

- Implement Welford's algorithm to compute stddev
- It allows us to compute a running stddev without having to store the previous data as well. 

## References:

- https://jonisalonen.com/2013/deriving-welfords-method-for-computing-variance/
- http://www.johndcook.com/blog/2008/09/26/comparing-three-methods-of-computing-standard-deviation/


The respective language folders' readmes contain implementation details.

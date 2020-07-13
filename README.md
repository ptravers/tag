## Tag
The board will show an aggregate of the number of agents at each location in the 4x4 grid.
The agents are all playing tag and randomly running. If an agent is it they will
always tag a player unless none is nearby.

The simulation is updated in parallel, mostly. The final update that moves agents
to their final destination is not done in parallel. The issue here was that to do
so with Rust's Sync and Send semantics, I would need to rewrite the logic or add
a mutex on the board which would result in the same or worse performance as not parallelizing at all.

The simulation could be trivially extended to support larger and smaller boards.

I ran out of time whilst starting the process of profiling the system with
a flamegraph to start looking for more optimal ways to maintain the board.

### Installation
Requires that cargo be installed and upto date. Additionally please ensure that
you have an internet connection as it requires the downloading of two libraries.

### Run
```
make run
```
to exit please use `ctrl c`
### Test
```
make test
```
to run benchmark
```
make bench
```

### Parallelism
The current implemementation is pretty poor for parallelism. I noted that the bench
results showed about a 10x performance decrease when the system was converted to
run in parallel. I suspect that this is due to the cost of scheduling overwhelming
the cost of manipulating the small number of elements. Additionally when running
computations on list of sixteen elements or less the computation should use SIMD.
As of writing I am unsure the full process by which rust will convert an iterator
operation to x86 intructions. Mostly as there are two major layers of software between
myself and the binary code. One being Rustc and the other being LLVM.

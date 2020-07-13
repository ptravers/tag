## Tag

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

### Parallelism
The current implemementation is pretty poor for parallelism. I noted that the bench
results showed about a 10x performance decrease when the system was converted to
run in parallel. I suspect that this is due to the cost of scheduling overwhelming
the cost of manipulating the small number of elements. Additionally when running
computations on list of sixteen elements or less the computation should use SIMD.
As of writing I am unsure the full process by which rust will convert an iterator
operation to x86 intructions. Mostly as there are two major layers of software between
myself and the binary code. One being Rustc and the other being LLVM.

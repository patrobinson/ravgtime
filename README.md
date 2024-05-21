## avgtime

A tool to measure the time distribution of a command across multiple executions.

Inspired by the excellent tool [avgtime](https://github.com/jmcabo/avgtime), implemented in Rust.

Arguments:

`-r` Number of times to run a command.

`-q` Pipe command stdout and stderr to `/dev/null`.

`-h` Display a histogram.

`-c` Number of concurrent executions.


Usage:

```sh
$ avgtime -r5 "sleep 1"
Total time: 5064ms
Repetitions: 5
Average time: 1012ms
Min: 1008ms
Max: 1022ms
Standard deviation: 40.60788
p95: 1022ms
p99: 1022ms
```

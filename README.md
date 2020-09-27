## avgtime

A tool to measure the time distribution of a command across multiple executions.

Inspired by the excellent tool [avgtime](https://github.com/jmcabo/avgtime), implemented in Rust.

Arguments:

`-r` Number of times to run a command.

`-q` Pipe command stdout and stderr to `/dev/null`.

`-h` Display a histogram.

`-c` Number of concurrent executions.

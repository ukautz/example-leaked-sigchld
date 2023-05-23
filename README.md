# Leaked SIGCHLD issue

This repo contains an example that show-cases the consequences of the [`wait-timeout`](https://github.com/alexcrichton/wait-timeout) crate not removing the `SIGCHLD` handler. The [`console`](https://github.com/console-rs/console) crate is used to show how this leads to a scenario where anything that uses syscalls is prone to fail unless it handles `std::io::ErrorKind::Interrupted` errors correctly.

## How to reproduce

Execute `cargo run`, then send `kill -SIGCHLD <PID>` from another terminal. You will see something like the following:

```sh
$ cargo run
execution status code: Some(0)
send me a SIGCHLD on 3283052 plz
quit? (y/n) > Error: read from terminal
Caused by:
    Interrupted system call (os error 4)
```

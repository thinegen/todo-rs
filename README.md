# todo.rs

[![Rust](https://github.com/thinegen/todo-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/thinegen/todo-rs/actions/workflows/rust.yml) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Todo txt frontend, written in Rust.

It still has some quirks, so don't hesitate to open an issue if you find anything.

## Usage

```
t new [priority] <description>
t set <id> (prio|desc|proj|cat|est|act|stat) <value>
t rm  <id>|all
t ls [searchterm]
t clean
```

clean resets all the numbers, the rest does what it says.

Possible status:
Open, Backlog, Next, Planned, Doing, Review, Done

Colors:
Black, Red, Green, Yellow, Blue, Purple, Cyan, White

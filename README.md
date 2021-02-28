# todo.rs

[![Tests & Checks](https://github.com/thinegen/todo-rs/actions/workflows/checks.yml/badge.svg)](https://github.com/thinegen/todo-rs/actions/workflows/checks.yml) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Todo txt frontend, written in Rust.

It still has some quirks, so don't hesitate to open an issue if you find anything.

## Usage

```
t new [priority] <description>
t set (prio|desc|proj|cat|est|act|stat|color) <id> <value>
t rm  <id>|all
t do  <id>
t ls [searchterm]
t clean
```

clean resets all the numbers, the rest does what it says.

Possible status:
Open, Backlog, Next, Planned, Doing, Review, Done

Colors:
Black, Red, Green, Yellow, Blue, Purple, Cyan, White

### Todos

- Fix those many many writes
- alias some stuff (e.g. ```t pri 1 5```)
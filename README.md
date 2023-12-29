# My Advent of Code Solutions in Rust

This repository contains my solutions for the Advent of Code challenges, written as I learn Rust. The project structure is based on a pre-made Rust template specifically designed for Advent of Code.

Each day's challenge is solved by a `solve()` function that returns a pair of `Solution`. The `Solution` is an enum that can hold any integer or a string.

A `Solution` can be created by specifying its type, such as `Solution::U32(value)`, or by using the From trait, which is implemented for all supported types. For instance, `Solution::from(value)`.

To execute the solutions for specific days, use the following command: `cargo run --release [days...]`

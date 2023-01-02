dumpgpt-rs ([v0.1.0](https://github.com/kusumi/dumpgpt-rs/releases/tag/v0.1.0))
========

## About

+ Parse and dump GPT in ASCII text.

+ Rust version of [https://github.com/kusumi/dumpgpt-go](https://github.com/kusumi/dumpgpt-go).

## Requirements

Recent version of Rust

## Build

    $ make

or

    $ gmake

## Usage

    $ ./target/debug/dumpgpt-rs
    usage: ./target/debug/dumpgpt-rs [<options>] <paths>
    
    Options:
            --verbose       Enable verbose print
            --symbol        Print symbol name if possible
            --noalt         Do not dump secondary header and entries
        -v, --version       Print version and exit
        -h, --help          Print usage and exit

## Resource

[https://github.com/kusumi/dumpgpt-rs](https://github.com/kusumi/dumpgpt-rs)

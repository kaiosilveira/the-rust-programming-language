# Chapter 9 - Error handling: Notes

- Our program can panic directly via the `panic!` macro or indirectly because of incorrect / unsafe behavior;
- enabling `RUST_BACKTRACE=1` will make the program print the backtrace in case of a panic;

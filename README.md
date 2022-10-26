# The Rust Programming Language

This repo contains my learnings from "The Rust Programming Language" book, with code snippets and small projects added as I was reading it.

## Basic commands

**rustc**

`rustc` stands for "Rust Compiler" and is the command used to compile Rust code into an executable binary.

| Command         | Description                 |
| --------------- | --------------------------- |
| `rustc main.rs` | compiles the code           |
| `./main`        | runs the generated binaries |

**cargo**

Cargo is the package manager and build system for Rust.

| Command                  | Description                                                                                   |
| ------------------------ | --------------------------------------------------------------------------------------------- |
| `cargo new project_name` | creates a project                                                                             |
| `cargo build`            | builds the project                                                                            |
| `cargo run`              | builds and runs the project                                                                   |
| `cargo check`            | checks whether the code compiles or not, faster than cargo build and doesn’t produce binaries |

## Main takeaways

Rust is a language created to be safe and performant at the same time, attributes that often do not go together in the same sentence. Historically, we could have either high-performance (C/C++) or safety (C#, Java), with some weird\* things in the middle (Javascript, Ruby). Rust aims to change this scenario by bringing memory safeguards and immutability by default into the language design, together with strong, semantic and verbose static-checks at compile time and modern tooling to help we accomplish our tasks. These compile checks are solid enough to remove the need of a garbage collector (GC), which improves overall performance manyfold.

\* I do actually love these two languages a lot, so I feel comfortable enough to call them weird

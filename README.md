![Language: Rust](https://img.shields.io/badge/language-Rust-green.svg)
![Topic: Academic](https://img.shields.io/badge/topic-Academic-red.svg)
![Topic: Interpreters](https://img.shields.io/badge/topic-Interpreters-green.svg)
![Status: In Progress](https://img.shields.io/badge/status-Done-green.svg)

# Primitive Bytecode Interpreters

The five branches here represent [Vladimir
Kazanov](https://github.com/vkazanov/)'s first five [World's Simplest
Bytecode
Interpreters](https://github.com/vkazanov/bytecode-interpreters-post/blob/master/README.org),
starting with one that has exactly two instruction and ending with one
that does something useful, in this case recognize Kleene regular
expressions *without* closure (that is, without the `*` operator).
Kazanov's versions are in C and have all the advantages and
disadvantages of C; these are in Rust, and the tradeoffs between
efficiency and safety are described in a series of blog posts:

* [The World's Simplest Bytecode Interpreter](https://elfsternberg.com/2019/04/17/worlds-simplest-bytecode-interpreter/)
* [The World's Second Simplest Bytecode Interpreter](https://elfsternberg.com/2019/04/17/the-second-simplest-bytecode-interpreter-in-rust/)
* [The World's Third Simplest Bytecode Interpreter: A Stack Machine](https://elfsternberg.com/2019/04/17/the-third-simplest-bytecode-interpreter-in-rust/)
* [The World's Fourth Simplest Bytecode Interpreter: A Register Machine](https://elfsternberg.com/2019/04/18/worlds-fourth-simplest-bytecode-interpreter-register-machines/)
* [The World's Fifth Simplest Bytecode Interpreter: A Regular Expression Engine](https://elfsternberg.com/2019/04/18/the-worlds-fifth-simplest-bytecode-interpreter-a-regular-expression-engine/)

The biggest lesson is that Rust's literalness with its `enum` uses more
memory and takes longer to load a program off-disk, but is much easier
to reason about and modify.  Rust's memory management makes some things
impossible, but doesn't constrain you from walking off the edge of the
world in a program spine, a data sample, or a stack; on the other hand,
while you can represent a syntactically invalid bytecode program in C,
it's impossible to do the same thing in Rust.

## License

GPLv3.  Consult the [LICENCE](./LICENSE) file for details.

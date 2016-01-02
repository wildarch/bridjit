# bridjit, the snobby brainfuck JIT
A JIT compiler for [brainfuck](https://en.wikipedia.org/wiki/Brainfuck), written in Rust.

This is a personal research project on how just-in-time compilation might work. It is highly unpractical, but perhaps suitable for educational purposes.

The project has reached all goals I had when building it, so I probably won't be adding many new features to it. However, should you happen to find a bug somewhere, please let me know!

# Installation

    cargo install --git https://github.com/wildarch/bridjit
    
# Usage

    echo "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++." > hello.b
    bridjit hello.b
Prints "Hello, world!".

A set of brainfuck example programs to toy with are in 'bf_src'.

# License
MIT license.

I did not write any of the example programs. Most of them are courtesy of [Daniel Cristofani](http://www.hevanet.com/cristofd/brainfuck/). I could not find a license on his website, but I will take them down immediately if I  violate some copyright/license by providing them here.

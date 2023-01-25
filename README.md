# cryptoquip

This repository contains a simple Rust program to help solve the
"Cryptoquip" newspaper puzzle.  This is syndicated by [King
Features](http://kingfeatures.com/) as [Sheffer Crossword with
Cryptoquip](http://kingfeatures.com/features/puzzlesandgames/sheffer-crossword/)
(a few days worth is available at
[cecildaily](http://www.cecildaily.com/diversions/cryptoquip/)).  This
is _not_ an automated solver, since that completely eliminates any fun
to be had.

The program is non-graphical.  The console mode commands are terse --
single letters -- but I found the user-interface to be quite
effective.  Type in `?` to get a list of commands.  The only painful
part of using it is that one has to actually type in the cryptogram --
using a camera to take an image and then performing optical character
recognition might be reasonable, but probably not worth the effort.

## How To Build

```
$ git clone git@github.com:bennetyee/cryptoquip.git
$ cd cryptquip
$ cargo build
```

## How To Run

The interface is very primitive:

```
$ target/debug/cryptoquip
jwblqr bx snbswn utb otgrh aynqhxqjo inynqw aygrlj tqssgrnjj: "wgxn gj q abuw bx itnnygbj."
cq: r ba
..a... a. ..a... ..a ..... ......... ...... ...... .........: ".... .. . .a.. a. ......a.."
cq: r xt   
..a... at ..a... ..a ..... .....t... ...... ...... .........: "..t. .. . .a.. at ......a.."
cq: p
jwblqr bx snbswn utb otgrh aynqhxqjo inynqw aygrlj tqssgrnjj: "wgxn gj q abuw bx itnnygbj."
..a... at ..a... ..a ..... .....t... ...... ...... .........: "..t. .. . .a.. at ......a.."
cq: ?
?            - this help
p            - print ciphertext / decoded text
N ciphertext - new cryptoquip with ciphertext
c            - clear mappings
r abcd...    - replace a with b, c with d, etc
               post-image can be "." to remove
               the current guess for the pre-image
s            - show current mapping
q            - quit
cq:
```



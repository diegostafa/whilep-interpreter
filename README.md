# Whilep

An interpreter for the concrete and abstract semantics of the While+ programming language

## Language

categories:
* *n* : Integer
* *x* : Variable
* *a* : Arithmetic expression
* *b* : Boolean expression

grammar:
```antlr
S ::= skip
    | x := a | x op= a | x++ | x--
    | if b then S else S end
    | while b do S done
    | repeat S until b
    | for x in [a, a] do S done

a ::= n
    | [a, a]
    | x
    | (a op a)
    | x++ | x--

b ::= true
    | false
    | !b
    | (b logic b)
    | (a compare a)

op      ::= + | - | * | /
logic   ::= && | ||
compare ::= == | != | < | > | <= | >=
```

## Usage

```
Usage: whilep [OPTIONS] --source-file <SOURCE_FILE>

Options:
  -s, --source-file <SOURCE_FILE>  Path to the source file
      --eval                       Perform a concrete evaluation
      --check-interval             Perform an abstract evaluation on the interval domain
      --check-constant             Perform an abstract evaluation on the constant domain
  -b, --bounds <BOUNDS>            Set the lower and upper bounds for the interval domain
  -h, --help                       Print help
  -V, --version                    Print version
  ```
## Build and run

```sh
cargo run -- -s example/test.wp --check-interval --check-constant --eval
```
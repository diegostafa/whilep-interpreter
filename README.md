# Whilep

An interpreter for the concrete and abstract semantics of the While+ programming language

## Language

categories:
* *n* : Integer
* *range* : Interval
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
    | for x in range do S done

a ::= n
    | range
    | x
    | (a op a)
    | x++ | x--

b ::= true
    | false
    | ! b
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

output:
```
[INFO] evaluating the abstract semantics in the Interval domain
+---+--------------------------+-------------------+
| # | Program point            | Invariant         |
+---+--------------------------+-------------------+
| 0 | x := 10                  | x: [10]           |
| 1 | y := 7                   | x: [10], y: [7]   |
| 2 | [while-guard] (x-- >= 0) | x: [-1,9], y: [7] |
| 3 | y := (y + 1)             | x: [-1,9], y: [8] |
| 4 | y := (y - 1)             | x: [-1,9], y: [7] |
| 5 | [end-while] (x-- < 0)    | x: [-2], y: [7]   |
+---+--------------------------+-------------------+

[INFO] evaluating the abstract semantics in the Constant domain
+---+--------------------------+--------------+
| # | Program point            | Invariant    |
+---+--------------------------+--------------+
| 0 | x := 10                  | x: 10        |
| 1 | y := 7                   | x: 10, y: 7  |
| 2 | [while-guard] (x-- >= 0) | x: Any, y: 7 |
| 3 | y := (y + 1)             | x: Any, y: 8 |
| 4 | y := (y - 1)             | x: Any, y: 7 |
| 5 | [end-while] (x-- < 0)    | x: Any, y: 7 |
+---+--------------------------+--------------+

[INFO] evaluating the concrete semantics
+---+-----+-----+
| # | Var | Val |
+---+-----+-----+
| 0 | y   | 7   |
| 1 | x   | -2  |
+---+-----+-----+
```
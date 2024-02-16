# Whilep

An interpreter for the concrete and abstract semantics of the While+ programming language

## Language

catgories:
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

## Example

```
x := 10; y := 0;
while (x >= 0) do x--; y++ done
```

output:
```
+---+------------------------+--------------+
| # | Program point          | Invariant    |
+---+------------------------+--------------+
| 0 | x := 10                | x: 10        |
| 1 | y := 7                 | x: 10, y: 7  |
| 2 | [while-guard] (x >= 0) | x: Any, y: 7 |
| 3 | x := (x - 1)           | x: Any, y: 7 |
| 4 | y := (y + 1)           | x: Any, y: 8 |
| 5 | y := (y - 1)           | x: Any, y: 7 |
| 6 | [end-while] (x < 0)    | x: Any, y: 7 |
+---+------------------------+--------------+
```
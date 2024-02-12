## Whilep

An interpreter for the concrete and abstract semantics of the While+ programming language


## Example

input:
```
x := 100;
y := 22;

while (x >= 0) do
    if (x <= y) then
        while (x >= 10) do x-- done
    else
        y++
    end
done
```

output:
```
+----+-------------------------+-------------------------------+
| #  | Program point           | Invariant                     |
+----+-------------------------+-------------------------------+
| 0  | x := 100                | x: [100]                      |
| 1  | y := 22                 | x: [100], y: [22]             |
| 2  | [while-guard] (x >= 0)  | x: [0, 100], y: [22, posinf]  |
| 3  | [if-guard] (x <= y)     | x: [0, 100], y: [22, posinf]  |
| 4  | [while-guard] (x >= 10) | x: [10, 100], y: [22, posinf] |
| 5  | x := (x - 1)            | x: [9, 99], y: [22, posinf]   |
| 6  | [end-while]             | x: [0, 9], y: [22, posinf]    |
| 7  | [else-guard] !(x <= y)  | x: [23, 100], y: [22, 99]     |
| 8  | y := (y + 1)            | x: [23, 100], y: [23, 100]    |
| 9  | [end-if]                | x: [0, 100], y: [22, posinf]  |
| 10 | [end-while]             | BOTTOM STATE                  |
+----+-------------------------+-------------------------------+
```
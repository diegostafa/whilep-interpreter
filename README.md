## Whilep

An interpreter for the denotational semantic of the While+ programming language

syntax:
```
statements:
      skip
    | x := Aexp
    | if Bexp then S1 else S2 end
    | while Bexpr do S end
    | for x in [Aexp..Aexp] do S end
    | repeat S until Bexp

```
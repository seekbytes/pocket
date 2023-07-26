# POCKET

[POCKET](https://github.com/seekbytes/pocket) is a program for applying transformations to obfuscate an MBA (Mixed Boolean Arithmetic) expression. It accepts input expressions and obfuscates them through some rules that are applied by substitution. E.g. the expression `A + B` is modified to `((A & B) + (A | B))`.

Examples of rules that are applied:

```
X ^ Y == (X | Y) - (X & Y)
X + Y == (X & Y) + (X | Y)
X - Y == (X ^ -Y) + 2*(X & -Y)
X & Y == (X + Y) - (X | Y)
X | Y == X + Y + 1 + (~X | ~Y)
``` 

## Making static analysis difficult

Most decompilers (e.g., IDA Pro, Ghidra, Binary Ninja) are able to regain most of the original instructions from any binary program. Although machine-coded instructions do not contain extensive information (such as comments, variable names, high-level structures), it is still possible to recover much of the original logic.

Making the analysis performed by decompilers difficult is a task based on the concept of obfuscation. Given a program P as input, the obfuscation operation tries to apply some techniques to complicate the program's logic, hindering the work done by decompilers. One of the simplest techniques to implement is the MBA operation obfuscation technique.

MBA expressions are expressions that include logical and arithmetic operations. Examples of MBA expressions are given by the result of decompilers attempting to transform a sequence of machine language operations into a logical, arithmetic expression. Generally MBA expressions are used to encode a logical number or condition by the compiler optimizer.

```
rax = 0x8007afb + (0x780 >> 2) | 0x87
```

Transformations are applied by rewriting the syntax tree constructed from the expression. The rewriting is simply a visit to the tree from the root node that recursively applies the transformation rules to obfuscate individual nodes. For example, the expression `A + (B & C)` produces a syntactic tree consisting of:

```
.
|- +
|--- A
|--- &
|------ C
|------ D
```

We apply the transformation `LEFT + RIGHT = (LEFT & RIGHT) + (LEFT | RIGHT)`. We obviously have to be careful what we have on the left and right. For the example `LEFT` is `A`, while `RIGHT` is `C & D`. The transformation rewrites the expression, making it: `(A & (C & D)) + (A | (C & D))`. End. Applying the different transformations and making them even "heavier" algebraically speaking (the value `1` can be rewritten in infinite ways, including `(((-1283928202102 & 1283928202103) << 2) >> 2) + (((-1283928202102 | 1283928202103) << 2) >> 2)`). There is no limit to the imagination!

The trick then is to use the result of the first obfuscation (level #0) as input for the next level of obfuscation and on and on. You can continue to make "passes" on the original expression to arrive at an expression that is more and more complex (and with many more nodes!). Everything has a price, however: increasing the level of obfuscation increases both time (it takes longer to traverse the tree and rewrite) and space (the abstract syntax tree takes up more bytes, trivially). For reasons unknown to me, on large expressions, the project struggles to apply transformations.

To check whether an obfuscated expression is semantically the same as the original expression, I wrote a simple interpreter that involves evaluating the expression as a kind of calculator. If the result obtained from evaluating the obfuscated expression is equal to the result of the original expression, then the transformation is valid and the expression has been obfuscated correctly. The value of the SET is determined by the value in the ASCII table of the SET name (e.g. 'A' is 65). 

## Compile and run the project.

The project is written in Rust using the [Pest](https://pest.rs) parser. Pest allows you to write a grammar using the syntax for [parser expression grammars](https://en.wikipedia.org/wiki/Parsing_expression_grammar) (PEG). The language grammar is available in the file `src/grammar.pest` and describes the type of expressions accepted within the program. Here we summarize a couple of rules:

- an expression consists of one or a series of logical and arimetic operations described as follows:
	- binary operations: SET OP SET, where SET is a letter of the ASCII alphabet while OP is between - (subtraction), + (addition), | (OR), & (AND), ^ (XOR)
	- unary operation: OP SET, where SET is a letter of the ASCII alphabet while OP is between ~ (NOT) and - (negation operation)

To compile the project simply use the `cargo build` command, you will find the binary inside `./target/debug/mfa`. To run it instead `cargo run`.

## Why is the project called POCKET?

The name POCKET ("pocket" in Italian) comes from an everyday life pill. How many times have we happened to insert a skein of a wire (earphones, charger) inside a pocket and after pulling it out we found ourselves with a wire to unravel? Too many times.

Here, let's imagine we have an expression (the thread) and we want to make it more complex to visualize. We can use POCKET (pocket) to have a more complex expression in a magical way. We simply insert it in and after a handful of microseconds we have our more complex expression.

Whatever, I had no other titles and this seemed like a good idea.

## Example

Input: `A + B`

Obfuscated version 1: `((A & B) + (A | B))`

Obfuscated version 2: `((((A + B) - (A | B)) & (A + (B + (1 + ((~A) | (~B)))))) + (((A + B) - (A | B)) | (A + (B + (1 + ((~A) | (~B)))))))`

Obfuscated version 3: ```
((((((((A & B) + (A | B)) ^ (-(A + (B + (1 + ((~A) | (~B))))))) + (2 * (((A & B) + (A | B)) & (-(A + (B + (1 + ((~A) | (~B))))))))) + ((A & ((B & (((593517776 - 593517775) & ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))) + ((593517776 - 593517775) | ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))))) + (B | (((593517776 - 593517775) & ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))) + ((593517776 - 593517775) | ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))))))) + (A | ((B & (((593517776 - 593517775) & ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))) + ((593517776 - 593517775) | ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))))) + (B | (((593517776 - 593517775) & ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))) + ((593517776 - 593517775) | ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))))))))) - (((((A & B) + (A | B)) ^ (-(A + (B + (1 + ((~A) | (~B))))))) + (2 * (((A & B) + (A | B)) & (-(A + (B + (1 + ((~A) | (~B))))))))) | ((A & ((B & (((593517776 - 593517775) & ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))) + ((593517776 - 593517775) | ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))))) + (B | (((593517776 - 593517775) & ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))) + ((593517776 - 593517775) | ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))))))) + (A | ((B & (((593517776 - 593517775) & ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))) + ((593517776 - 593517775) | ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))))) + (B | (((593517776 - 593517775) & ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))) + ((593517776 - 593517775) | ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B)))))))))))))) & (((((A & B) + (A | B)) ^ (-(A + (B + (1 + ((~A) | (~B))))))) + (2 * (((A & B) + (A | B)) & (-(A + (B + (1 + ((~A) | (~B))))))))) + (((A & ((B & (((756408559 - 756408558) & ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))) + ((756408559 - 756408558) | ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))))) + (B | (((756408559 - 756408558) & ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))) + ((756408559 - 756408558) | ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))))))) + (A | ((B & (((756408559 - 756408558) & ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))) + ((756408559 - 756408558) | ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))))) + (B | (((756408559 - 756408558) & ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))) + ((756408559 - 756408558) | ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B)))))))))))) + (1 + ((~((((A & B) + (A | B)) ^ (-(A + (B + (1 + ((~A) | (~B))))))) + (2 * (((A & B) + (A | B)) & (-(A + (B + (1 + ((~A) | (~B)))))))))) | (~((A & ((B & (((756408559 - 756408558) & ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))) + ((756408559 - 756408558) | ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))))) + (B | (((756408559 - 756408558) & ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))) + ((756408559 - 756408558) | ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))))))) + (A | ((B & (((756408559 - 756408558) & ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))) + ((756408559 - 756408558) | ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))))) + (B | (((756408559 - 756408558) & ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))) + ((756408559 - 756408558) | ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B)))))))))))))))))) + (((((((A & B) + (A | B)) ^ (-(A + (B + (1 + ((~A) | (~B))))))) + (2 * (((A & B) + (A | B)) & (-(A + (B + (1 + ((~A) | (~B))))))))) + ((A & ((B & (((593517776 - 593517775) & ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))) + ((593517776 - 593517775) | ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))))) + (B | (((593517776 - 593517775) & ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))) + ((593517776 - 593517775) | ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))))))) + (A | ((B & (((593517776 - 593517775) & ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))) + ((593517776 - 593517775) | ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))))) + (B | (((593517776 - 593517775) & ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))) + ((593517776 - 593517775) | ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))))))))) - (((((A & B) + (A | B)) ^ (-(A + (B + (1 + ((~A) | (~B))))))) + (2 * (((A & B) + (A | B)) & (-(A + (B + (1 + ((~A) | (~B))))))))) | ((A & ((B & (((593517776 - 593517775) & ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))) + ((593517776 - 593517775) | ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))))) + (B | (((593517776 - 593517775) & ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))) + ((593517776 - 593517775) | ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))))))) + (A | ((B & (((593517776 - 593517775) & ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))) + ((593517776 - 593517775) | ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))))) + (B | (((593517776 - 593517775) & ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))) + ((593517776 - 593517775) | ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B)))))))))))))) | (((((A & B) + (A | B)) ^ (-(A + (B + (1 + ((~A) | (~B))))))) + (2 * (((A & B) + (A | B)) & (-(A + (B + (1 + ((~A) | (~B))))))))) + (((A & ((B & (((756408559 - 756408558) & ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))) + ((756408559 - 756408558) | ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))))) + (B | (((756408559 - 756408558) & ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))) + ((756408559 - 756408558) | ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))))))) + (A | ((B & (((756408559 - 756408558) & ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))) + ((756408559 - 756408558) | ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))))) + (B | (((756408559 - 756408558) & ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))) + ((756408559 - 756408558) | ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B)))))))))))) + (1 + ((~((((A & B) + (A | B)) ^ (-(A + (B + (1 + ((~A) | (~B))))))) + (2 * (((A & B) + (A | B)) & (-(A + (B + (1 + ((~A) | (~B)))))))))) | (~((A & ((B & (((756408559 - 756408558) & ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))) + ((756408559 - 756408558) | ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))))) + (B | (((756408559 - 756408558) & ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))) + ((756408559 - 756408558) | ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))))))) + (A | ((B & (((756408559 - 756408558) & ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))) + ((756408559 - 756408558) | ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))))) + (B | (((756408559 - 756408558) & ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B))))))) + ((756408559 - 756408558) | ((~A) + ((~B) + (1 + ((~(~A)) | (~(~B)))))))))))))))))))
```


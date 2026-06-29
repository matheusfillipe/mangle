[![Website](https://img.shields.io/badge/website-mangle.ga-blue.svg)](https://mangle.ga)
[![Pypi](https://badge.fury.io/py/mangle.svg)](https://pypi.org/project/mangle/)
[![Crates.io](https://img.shields.io/crates/v/mangle.svg)](https://crates.io/crates/mangle)
[![Chat with me on irc](https://img.shields.io/badge/-IRC-gray?logo=gitter)](https://mangle.ga/irc)
[![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/matheusfillipe/mangle/master/LICENSE)
[![CircleCI Build Status](https://circleci.com/gh/matheusfillipe/mangle.svg?style=shield)](https://circleci.com/gh/matheusfillipe/mangle)
[![codecov](https://codecov.io/gh/matheusfillipe/mangle/branch/master/graph/badge.svg)](https://codecov.io/gh/matheusfillipe/mangle)

# Mangle

An exolang (a scripting programming language just for fun) without any reserved keywords that can run any utf8 text as valid code. Meaning is derived entirely from word lengths and punctuation, never from the words themselves.

## Installation

`cargo install mangle`

### Python bindings

Python bindings are available for python > 3.7. You can install them with:

`pip install mangle`

Then you can evaluate mangle from python:

```python
import mangle
mangle.eval("cat is fat")
>> '5'
```

More info at: https://github.com/matheusfillipe/mangle/tree/master/python

## How it works

There are only labels, variables and operators. All variables are globally scoped within a single program; there are no locals or classes. All variables are dynamically typed and shadowed on reassignment.

### Sentences

The only kind of scope is the sentence. Sentences are delimited by the punctuation marks `.,;:?!`. Any other token is interpreted as an operator, a variable name, or a literal. Text with no punctuation runs as a single sentence.

### Labels

A sentence's label is its last word before the punctuation mark — except for the first sentence, which is `main` and needs no label. Labels name subroutines you can jump to. The first sentence runs automatically; after it finishes, execution falls through to the next sentence in order, unless a jump redirects it.

### Data types

There are two types: **ints** and **stacks**.

A word's numeric value is its **grapheme length**: `"a"` evaluates to `1`, `"cat"` to `3`. A word is also a variable name; when a word is looked up as a value and is unbound, its length is used.

A stack is a `Vec<int>`. Pushing onto an unbound variable creates a fresh stack. Popping an empty stack yields `0`.

### The `it` register

After every operation the interpreter stores its result in the variable `it`. This is the only way to carry a computed value into a later operation (since assign only reads a single word). For example `take cat cat add it fat` subtracts `3 - 3 = 0` into `it`, then adds `it + fat = 0 + 3 = 3`.

## Operators

The first word of each clause is the operator, dispatched **by its grapheme length, nothing else**. There are no keywords: `is`, `be`, `as`, `of` — every 2-letter word — is the assign operator, and so on. Pick whichever word of the right length makes your prose read best; the word's meaning is irrelevant, only its length matters. Lengths are assigned by frequency: the most common English word lengths (3, 4, 2, 5, 6 ...) map to the most-used operators, so natural prose tends to compile into useful programs. The operator consumes as many following words as its arity, then the next word is another operator, and so on until the sentence ends.

| Word length | Operator  | Arity | Example word (any word this length works) | Effect                                   |
|:-----------:|:---------:|:-----:|:-----------------------------------------:|:----------------------------------------:|
| 1           | literal   | 1     | `a`                                       | evaluates to its argument's length       |
| 2           | assign    | 2     | `is`                                      | `x := value`                             |
| 3           | sum       | 2     | `add`                                     | `a + b`                                  |
| 4           | if-zero   | 2     | `when`                                    | jump when value is `0`                   |
| 5           | goto      | 1     | `leaps`                                   | jump to a label                          |
| 6           | print     | 1     | `output`                                  | prints a number                          |
| 7           | subtract  | 2     | `reduces`                                 | `a - b`                                  |
| 8           | multiply  | 2     | `multiply`                                | `a * b`                                  |
| 9           | divide    | 2     | `quotients`                               | `a / b`                                  |
| 10          | pop       | 1     | `everything`                              | pop the top of a stack                   |
| 11          | push      | 2     | `environment`                             | push a value onto a stack                |
| 12          | modulo    | 2     | `subsequently`                            | `a % b`                                  |
| 13          | puttext   | 1     | `communicating`                           | prints one literal word                  |
| 14          | phrase    | rest  | `affectionately`                          | prints the rest of the sentence as text  |

Division by zero, unknown labels, unknown operator lengths, arity mismatches and infinite loops (capped at one million operations) all raise errors.

## But then spaces are keywords heh?

Well... In some interpretation yes, I guess I lied then, sorry about it. You can still pass the `-F` argument to change the word separator (like field separator in awk) to any other character.

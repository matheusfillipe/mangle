[![Pypi](https://badge.fury.io/py/mangle.svg)](https://pypi.org/project/mangle/)
[![Crates.io](https://img.shields.io/crates/v/mangle.svg)](https://crates.io/crates/mangle)
[![Chat with me on irc](https://img.shields.io/badge/-IRC-gray?logo=gitter)](https://mangle.ga/irc)
[![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/matheusfillipe/mangle/master/LICENSE)
[![CircleCI Build Status](https://circleci.com/gh/matheusfillipe/mangle.svg?style=shield)](https://circleci.com/gh/matheusfillipe/mangle)
[![codecov](https://codecov.io/gh/matheusfillipe/mangle/branch/master/graph/badge.svg)](https://codecov.io/gh/matheusfillipe/mangle)

### WIP: This is a work in progress, there is nothing working yet

# Mangle

An exolang (A scripting programming language just for fun) without any reserved keywords that can run any utf8 compatible with more than 2 space separated words on it.

## Installation

`cargo install mangle`


### Python bindings

Python bindings are available for python > 3.7. You can install them with:

`pip install mangle`

Then you can get started evaluating mangle from python with:

```python
import mangle
mangle.eval("cat is fat")
>> '5'
```
More info at: https://github.com/matheusfillipe/mangle/tree/master/python

## How it works
There are only labels, variables and operators. All variables are globally scoped, there are no locals, classes or anything fancy. All variables are dynamically typed and shadowed.

### Sentences
The only type of scope is sentences. Sentences are like english sentences, any text that comes before the punctuation marks: `.,;:?!`. All the other symbols will be interpreted as variable names, operators, labels, strings or numbers.

If a text has none of those punctuation marks it will run as a single sentence.

### Sentence Labels
The label of a sentence is the last word before the punctuation mark except by the first sentence. Labels are the way to have subroutines on this language. They define a scope with a body that you can `goto` from any other subroutine. 

The first sentence on the interpreted code is the equivalent of the main function in another languages, so it has no need for a label. If the first sentence doesn't call any function, all the others labels will execute in the order they are until you exit or jump to another label.


### Data Types
The only types are strings, ints and stacks.

#### Int
The `length of a word - 1` define its numerical value. For example "a" evaluates to 0 and "cat" to 2.

#### Strings
The words themselves can be also interpreted as strings. You can't easily build multiword strings like "a bird" though since that would read as: `"a` and `bird"` each as individual words.

#### Stacks
Strings themselves are a stack of ints that the interpreter itself decodes at runtime. You can add ints to a stack by adding to its variable. 

WIP...


## Operator
Operators are defined by the word's length. Here is the table of operators on this language

| Word Length | Operator | syntax                      | Example    | Description                     |
|:-----------:|:--------:|:---------------------------:|:----------:|:-------------------------------:|
| 1           |          |                             |            |                                 |
| 2           | Assign   | _OP_ **receiver** **Value** | is cat fat | Assigns variable cat to value 3 |
| 3           |          |                             |            |                                 |
| 4           |          |                             |            |                                 |
| 5           |          |                             |            |                                 |
| 6           |          |                             |            |                                 |
| 7           |          |                             |            |                                 |
| 8           |          |                             |            |                                 |
| 9           |          |                             |            |                                 |
| 10          |          |                             |            |                                 |
| 11          |          |                             |            |                                 |


## But then spaces are keywords heh?
Well... In some interpretation yes, I guess I lied then, sorry about it. You can still pass the `-F` argument to change the word separator (like field separator in awk) to any other character.

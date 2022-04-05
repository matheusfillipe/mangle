[![Crates.io](https://img.shields.io/crates/v/mangle.svg)](https://crates.io/crates/mangle)
[![Chat with me on irc](https://img.shields.io/badge/-IRC-gray?logo=gitter)](https://mangle.ga/irc)
[![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/matheusfillipe/mangle/master/LICENSE)
[![CircleCI Build Status](https://circleci.com/gh/matheusfillipe/mangle.svg?style=shield)](https://circleci.com/gh/matheusfillipe/mangle)
[![codecov](https://codecov.io/gh/matheusfillipe/mangle/branch/master/graph/badge.svg)](https://codecov.io/gh/matheusfillipe/mangle)
# Mangle

These are the python bindings for mangle, an exolang (A scripting programming language just for fun) without any reserved keywords that can run any utf8 compatible with more than 2 space separated words on it.

### Python bindings

Python bindings are available for python > 3.7. You can install them with:

`pip install mangle`

Then you can get started evaluating mangle from python with:

```python
import mangle
mangle.eval("cat is fat")
>> '5'
```


## Learn more

You can read more about mangle at: https://github.com/matheusfillipe/mangle

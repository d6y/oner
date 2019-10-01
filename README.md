# A 1R implementation in Rust

Re-implementing the algorithm (and some experiments) described in Holte (1993).

## Status

- [x] Basic algorithm
- [ ] Support for missing values ([#1](https://github.com/d6y/oner/issues/1))
- [ ] Support for continuous values ([#2](https://github.com/d6y/oner/issues/2))
- [ ] Handle tied accuracy in rule discovery ([#3](https://github.com/d6y/oner/issues/3))

## Example run

 New to Rust? :wave: Start by [installing `rustup`](https://www.rust-lang.org/learn/get-started) to get the `cargo` command. Then...

```
$ cargo build --quiet --release
$ ./target/release/oner -d data/ch/ch.csv
```

## Background

See:

- Holte, R.C. _Machine Learning_ (1993) 11: 63. [https://doi.org/10.1023/A:1022631118932](https://doi.org/10.1023/A:1022631118932).

- Molnar, C, _Interpretable Machine Learning_ (2019). In particular: [Learn Rules from a Single Feature (OneR)](https://christophm.github.io/interpretable-ml-book/rules.html#learn-rules-from-a-single-feature-oner).

## Terminology

I'm following the terminology from Holte (1993):

- Attribute (a.k.a. feature)
- Value (the value of an attribute or class)
- Class (a.k.a. classification, prediction)
- Example (a.k.a. instance)

## Example data sets

I have taken data sets and converted to CSV where necessary, including adding header rows.

The `data` folder contains the data from various sources. Unless otherwise specified, it'll be the [UCI Machine Learning Repository](https://archive.ics.uci.edu/ml/citation_policy.html).

- `ch`, the [Chess (King-Rook vs. King-Pawn)](https://archive.ics.uci.edu/ml/datasets/Chess+%28King-Rook+vs.+King-Pawn%29) dataset.

- `fake-house`, the dataset used to introduce 1R in [Interpretable Machine Learning](https://christophm.github.io/interpretable-ml-book/rules.html#learn-rules-from-a-single-feature-oner) (published under [CC BY-NC-SA 4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/)). To run the example use the `-w` flag to use the whole dataset for rule discovery.



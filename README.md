# A 1R implementation in Rust

Re-implementing the algorithm (and some experiments) described in Holte (1993).

## Status

- [x] Basic algorithm
- [ ] Support for missing values ([#1](https://github.com/d6y/oner/issues/1))
- [ ] Support for continuous values ([#2](https://github.com/d6y/oner/issues/2))

## Example run

:wave: New to Rust? [Install `rustup`](https://www.rust-lang.org/learn/get-started) first.

```
$ cargo build --quiet --release
$ ./target/release/oner -d data/ch/ch.csv
```

## Background

See:

- Holte, R.C. _Machine Learning_ (1993) 11: 63. [https://doi.org/10.1023/A:1022631118932](https://doi.org/10.1023/A:1022631118932).

- [Learn Rules from a Single Feature (OneR)](https://christophm.github.io/interpretable-ml-book/rules.html#learn-rules-from-a-single-feature-oner),
  in [Interpretable Machine Learning](https://christophm.github.io/interpretable-ml-book/). You'll also find links there to [implementations in R, Python, Java](https://christophm.github.io/interpretable-ml-book/rules.html#software-and-alternatives).

## Terminology

I'm following the terminology from Holte (1993):

- Attribute (a.k.a. feature)
- Value
- Class (a.k.a. classification, prediction)
- Example (a.k.a. instance)

## Example data sets

I have taken data sets and converted to CSV where necessary, including adding header rows.

The `data` folder contains the following data from the [UCI Machine Learning Repository](https://archive.ics.uci.edu/ml/citation_policy.html):

- `ch`, the [Chess (King-Rook vs. King-Pawn)](https://archive.ics.uci.edu/ml/datasets/Chess+%28King-Rook+vs.+King-Pawn%29) dataset.



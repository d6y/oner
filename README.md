# A 1R implementation in Rust

Re-implementing the algorithm (and some experiments) described in Holte (1993).

## Status

- [x] Basic algorithm
- [ ] Support for missing values ([#1](https://github.com/d6y/oner/issues/1))
- [ ] Support for continuous values ([#2](https://github.com/d6y/oner/issues/2))

## Example run

```
TODO
```

## Background

See:

- Holte, R.C. _Machine Learning_ (1993) 11: 63. [https://doi.org/10.1023/A:1022631118932](https://doi.org/10.1023/A:1022631118932).

- [Learn Rules from a Single Feature (OneR)](https://christophm.github.io/interpretable-ml-book/rules.html#learn-rules-from-a-single-feature-oner),
  in [Interpretable Machine Learning](https://christophm.github.io/interpretable-ml-book/). You'll also find links there to [implementations in R, Python, Java](https://christophm.github.io/interpretable-ml-book/rules.html#software-and-alternatives).


## Example data sets

The `data` folder contains the following data from the [UCI Machine Learning Repository](https://archive.ics.uci.edu/ml/citation_policy.html):

- `ch`, the [Chess (King-Rook vs. King-Pawn)](https://archive.ics.uci.edu/ml/datasets/Chess+%28King-Rook+vs.+King-Pawn%29) dataset.



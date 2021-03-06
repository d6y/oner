![Rust](https://github.com/d6y/oner/workflows/Rust/badge.svg)

# A 1R implementation in Rust

Re-implementing the 1R experiments described in [Holte, 1993](https://doi.org/10.1023/A:1022631118932).

# 1R is a baseline rule learning algorithm.

The algorithm generates a rule for each attribute, and then picks the "one rule" that has the best accuracy.

For example, given a data set of drinking habits with attributes such as age, time of day, mood (attributes), 1R might produces a rule of the form:

```
if time="morning" then drink="coffee"
if time="afternoon" then drink="tea"
if time="evening" then drink="water"
```

The rule might only have, say, 60% accuracy.
That's a baseline to compare to other algorithms.

# Example run

New to Rust? :wave: Start by [installing `rustup`](https://www.rust-lang.org/learn/get-started) to get various tools, including the `cargo` command. Then...

```
$ cargo build --quiet --release
❯ ./target/release/oner -d data/fake-house/house.csv -w
Config { data: "data/fake-house/house.csv", seed: 1, training_fraction: 0.6666666666666666, hide_rules: false, use_whole_dataset: true, repeats: 25, distinct_above: 6, small: 6, missing: "?" }
// Training set accuracy: 0.70
IF size IS small THEN low
IF size IS big THEN high
IF size IS medium THEN medium
```

# Example data sets

This application assumes attributes (features) are the columns and rows are the instances (examples).

I have taken data sets and converted to CSV where necessary, including adding header rows.

The `data` folder contains the data from various sources. Unless otherwise specified, it'll be the [UCI Machine Learning Repository](https://archive.ics.uci.edu/ml/citation_policy.html).

## `bc`

A [breast cancer](https://archive.ics.uci.edu/ml/datasets/Breast+Cancer) dataset.

In the CSV version I have moved the class from the first column to the last column (that's what this code expects).
I did this with: `awk -F, '{print $2,$3,$4,$5,$6,$7,$8,$9,$10,$1}' OFS=, < breast-cancer.data > bc.csv`

Holt's experiments (section 2.2 of Holte 1993) used a random 2/3 of the data for training, and 1/3 for testing, repeated 25 times.
The experiment resulted in a 0.687 classification accuracy on the test (Holte, table 3) against a baseline (0R) accuracy of 0.73 (table 2).

| Model                          | Accuracy %  |
| ------------------------------ | ----------- |
| 0R                             | 70.3        |
| 1R                             | 68.7        |
| This code (mean of 10 seeds)   | 68.4        |
| This code (median of 10 seeds) | 68.3        |
| This code (range of 10 seeds)  | 67.6 - 69.6 |

## `ch`

The [Chess (King-Rook vs. King-Pawn)](https://archive.ics.uci.edu/ml/datasets/Chess+%28King-Rook+vs.+King-Pawn%29) dataset.

| Model                          | Accuracy %  |
| ------------------------------ | ----------- |
| 0R                             | 52.5        |
| 1R                             | 67.6        |
| This code (mean of 10 seeds)   | 67.6        |
| This code (median of 10 seeds) | 67.6        |
| This code (range of 10 seeds)  | 67.2 - 67.8 |

## `ir`

The [Iris](https://archive.ics.uci.edu/ml/datasets/Iris) dataset. The CSV version was created with:

```
$ echo "SepalLengthInCm,SepalWidthInCm,PetalLengthInCm,PetalWidthInCm,Class" > iris.csv
$ cat iris.data >> iris.csv
```

| Model                          | Accuracy %  |
| ------------------------------ | ----------- |
| 0R                             | 33.3        |
| 1R                             | 93.5        |
| This code (mean of 10 seeds)   | 95.1        |
| This code (median of 10 seeds) | 95.0        |
| This code (range of 10 seeds)  | 94.5 - 95.9 |

Using the whole data set:

```
❯ ./target/release/oner -d data/ir/iris.csv -w
Config { data: "data/ir/iris.csv", seed: 1, training_fraction: 0.6666666666666666, hide_rules: false, use_whole_dataset: true, repeats: 25, distinct_above: 6, small: 6, missing: "?" }
// Training set accuracy: 0.960
IF PetalWidthInCm IS < 1 THEN Iris-setosa
IF PetalWidthInCm IS >= 1 and < 1.7 THEN Iris-versicolor
IF PetalWidthInCm IS >= 1.7 THEN Iris-virginica
```

## `fake-house`

The dataset used to introduce 1R in [Interpretable Machine Learning](https://christophm.github.io/interpretable-ml-book/rules.html#learn-rules-from-a-single-feature-oner) (published under [CC BY-NC-SA 4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/)). To run the example use the `-w` flag to use the whole dataset for rule discovery.

# Configuration

```
oner 0.2.0

USAGE:
    oner [FLAGS] [OPTIONS] --data <filename>

FLAGS:
        --help                 Prints help information
    -h, --hide-rules           Suppress printing of rules at end of run
    -w, --use-whole-dataset    Use all the data for training and testing (overrides -t)
    -V, --version              Prints version information

OPTIONS:
        --distinct-above <distinct-above>
            An attribute must have more than than this number of distinct values for a column to be detected as numeric
            (and so quantized) [default: 6]
    -d, --data <filename>                          Complete data set to learn from (in CSV format, with header row)
    -m, --missing <missing>
            When quantizing, a value to treat as a missing value (in addition to blank attribute values) [default: ?]

    -r, --repeats <repeats>
            Number of times to repeat an experiment to report average accuracy [default: 25]

    -s, --seed <seed>                              Random seed [default: 1]
        --small <small>
            When quantizing, an interval must have a dominant class must occur more than this many times. [default: 6]

    -t, --training-fraction <training-fraction>
            Fraction of the data to use for training (vs. testing) [default: 0.6666666666666666]
```

# Licence

Copyright 2020 Richard Dallaway

This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at <https://mozilla.org/MPL/2.0/>.

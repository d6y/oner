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
IF size=small THEN low
IF size=big THEN high
IF size=medium THEN medium
```

# Example data sets

I have taken data sets and converted to CSV where necessary, including adding header rows.

The `data` folder contains the data from various sources. Unless otherwise specified, it'll be the [UCI Machine Learning Repository](https://archive.ics.uci.edu/ml/citation_policy.html).

## `bc`

A [breast cancer](https://archive.ics.uci.edu/ml/datasets/Breast+Cancer) dataset.

In the CSV version I have moved the class from the first column to the last column (that's what this code expects). 
I did this with: `awk -F, '{print $2,$3,$4,$5,$6,$7,$8,$9,$10,$1}' OFS=, < breast-cancer.data  > bc.csv`

Holt's experiments (section 2.2 of Holte 1993) used a random 2/3 of the data for training, and 1/3 for testing, repeated 25 times.
The experiment resulted in a 0.687 classification accuracy on the test (Holte, table 3) against a baseline (0R) accuracy of 0.73 (table 2).

This software produces a test accuracy between 0.673 and 0.693 (for seeds 1-10).

## `ch`
The [Chess (King-Rook vs. King-Pawn)](https://archive.ics.uci.edu/ml/datasets/Chess+%28King-Rook+vs.+King-Pawn%29) dataset.

## `cc`, the [Cervical cancer (risk Factors)](https://archive.ics.uci.edu/ml/datasets/Cervical+cancer+%28Risk+Factors%29) dataset. I've removed the Hinselmann, Schiller, and Cytology targets, leaving just the Biopsy target.

## `fake-house`

The dataset used to introduce 1R in [Interpretable Machine Learning](https://christophm.github.io/interpretable-ml-book/rules.html#learn-rules-from-a-single-feature-oner) (published under [CC BY-NC-SA 4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/)). To run the example use the `-w` flag to use the whole dataset for rule discovery.


# Configuration

```
oner 0.2.0

USAGE:
    oner [FLAGS] [OPTIONS] --data <filename>

FLAGS:
        --help                 Prints help information
    -h, --hide-rules           Supress printing of rules at end of run
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
            When quantizing, an interval must have a dominant class must occure more than this many times. [default: 6]

    -t, --training-fraction <training-fraction>
            Fraction of the data to use for training (vs. testing) [default: 0.6666666666666666]
```

# Licence

Copyright 2020 Richard Dallaway

This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at <https://mozilla.org/MPL/2.0/>.

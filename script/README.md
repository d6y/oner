# Utility scripts


## Many

Runs 1R ten times with a different seed (seeds 1-10).

If you have R (the programming language) installed,
you can summarise the runs like this:

```
 ./script/many.sh data/bc/bc.csv | grep accuracy | sort | cut -d':' -f 2 | Rscript -e 'print(summary(scan("stdin")));'
Read 10 items
   Min. 1st Qu.  Median    Mean 3rd Qu.    Max. 
 0.6750  0.6813  0.6820  0.6843  0.6887  0.6950 
```

Thank you [Unix Stack Excahnge](https://unix.stackexchange.com/questions/13731/is-there-a-way-to-get-the-min-max-median-and-average-of-a-list-of-numbers-in#comment243431_13775).
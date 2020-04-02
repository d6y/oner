#/bin/sh
# 
# Run 1R multiple times with different seeds
#
# Example Usage: ./script/many.sh data/bc/bc.csv

if [ $# -eq 0 ]
  then
    echo "Usage: $0 path-to-csv-file"
  else
    for seed in {1..10} 
    do 
        ./target/release/oner -d $1 -h -s $seed 
    done 
fi

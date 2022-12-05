#!/usr/bin/env bash

DAY=$1
FILEPATH="../src/bin/day_${DAY}.rs"

touch $FILEPATH

echo "use advent22::get_input_contents;" > $FILEPATH
echo "fn main() {" >> $FILEPATH
echo "    let file_contents = get_input_contents();" >> $FILEPATH
echo "}" >> $FILEPATH
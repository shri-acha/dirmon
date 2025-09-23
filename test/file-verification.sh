#!/bin/bash
for i in `seq 1 10000`
do 
	diff <(cat test_directory/test_file${i}.hash) <(sha256sum test_directory/Documents/test_file${i}.txt | awk '{print $1}')
done

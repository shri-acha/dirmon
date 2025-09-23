#!/bin/bash
echo "creating test directory"
mkdir test_directory

echo "integrity checks"
for val in `seq 1 10000`;
do
  touch test_directory/test_file${val}.txt &&\
  sha256sum test_directory/test_file${val}.txt | awk '{print $1}' > test_directory/test_file${val}.hash
  
done

#!/bin/bash

src_dir=$(cd ../../../ && pwd)

sed -e 's/^\(.*\)$/\/\/ \1/' $src_dir/c/loader.c > $src_dir/go/src/coleslaw/loader.temp

sed -e "/\/\/ \/\/ GENERATED CODE/r ${src_dir}/go/src/coleslaw/loader.temp" -e 's/###_C_CODE_HERE###//' $src_dir/go/src/coleslaw/template > $src_dir/go/src/coleslaw/loader.go
rm -f $src_dir/go/src/coleslaw/loader.temp

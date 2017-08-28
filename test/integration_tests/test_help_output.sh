#!/bin/bash

run_test() {
     local help_output=`$BIN --help 2>/dev/null`
     echo "checking that the --help flag outputs help and doesn't crash"
     echo $help_output | grep -e 'USAGE' >/dev/null
     return $?
}

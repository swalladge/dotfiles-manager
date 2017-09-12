#!/bin/bash

run_test() {
     echo "checking invalid arguments"
     out=$(exe -t /doesntexist/lol install something 2>&1)

     echo $out | grep -e 'Argument error: ' || return 1

     return 0
}

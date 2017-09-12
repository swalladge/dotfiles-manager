#!/bin/bash

run_test() {
     echo "checking invalid arguments"
     out=$(exe lololol --wat 2>&1)

     echo $out | grep -e 'error: Found argument' || return 1

     return 0
}

#!/bin/bash

run_test() {
     echo "checking that the --help flag outputs help and doesn't crash"
     exe --help
     return $?
}

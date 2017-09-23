#!/bin/bash

run_test() {
     echo "checking hook failed"
     out=$(exe -d "${BASE_DIR}/test/repo" -t "${TEMP_LOCAL}/" -B hook_fail_host -y install vim 2>&1)

     echo $out | grep -i -e 'failed.*hook.*Permission denied' || return 1

     return 0
}

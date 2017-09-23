#!/bin/bash

run_test() {
     out=$(exe -d "${BASE_DIR}/test/repo" -t "${TEMP_LOCAL}/" -B hook_fail_host2 -y install vim 2>&1)

     echo $out | grep -i -e 'hook failed with status code: 1' || return 1

     return 0
}

#!/bin/bash


run_test() {
     echo "checking that adding a file to a package works"

     # make some files/dirs
     echo "set compatible" > "${TEMP_LOCAL}/.vimrc2"
     echo "# hi" > "${TEMP_LOCAL}/.zshrc"

     exe -d "${BASE_DIR}/test/repo" -t "${TEMP_LOCAL}/" -B desktop1 add "${TEMP_LOCAL}/.vimrc2" -p vim

     # make sure it exited ok
     local last="$?"
     [[ "$last" != "0" ]] && return $last

     # TODO: implement in runner.rs and uncomment
     # assert_link "${TEMP_LOCAL}/.vimrc2" "${BASE_DIR}/test/repo/vim/files/.vimrc2" || return 1

     return 0
}

#!/bin/bash


run_test() {
     echo "checking that a general install of a package works as expected"
     exe -d "${BASE_DIR}/test/repo" -t "${TEMP_LOCAL}/" -B desktop1 install vim

     # make sure it exited ok
     local last="$?"
     [[ "$last" != "0" ]] && return $last

     # check the linked files exist
     assert_link "${TEMP_LOCAL}/.vimrc" "${BASE_DIR}/test/repo/vim/hosts/desktop1/files/.vimrc" || return 1
     assert_link "${TEMP_LOCAL}/.vim/filetime.vim" "${BASE_DIR}/test/repo/vim/files/.vim/filetype.vim" || return 1

     return 0
}

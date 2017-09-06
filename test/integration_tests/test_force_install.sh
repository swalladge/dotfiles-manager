#!/bin/bash


run_test() {
     echo "checking that a force install overwrites existing files"

     # make some files/dirs
     echo "set compatible" > "${TEMP_LOCAL}/.vimrc"
     mkdir -p "${TEMP_LOCAL}/.vim/filetype.vim"

     # run with force on
     exe -d "${BASE_DIR}/test/repo" -t "${TEMP_LOCAL}/" --force -B desktop1 install vim

     # make sure it exited ok
     local last="$?"
     [[ "$last" != "0" ]] && return $last

     # check files exist and now link to correct locations
     assert_link "${TEMP_LOCAL}/.vimrc" "${BASE_DIR}/test/repo/vim/hosts/desktop1/files/.vimrc" || return 1
     assert_link "${TEMP_LOCAL}/.vim/filetype.vim" "${BASE_DIR}/test/repo/vim/files/.vim/filetype.vim" || return 1

     return 0
}

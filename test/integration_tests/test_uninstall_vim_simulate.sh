#!/bin/bash


run_test() {
     echo "checking that a general install of a package works as expected"
     exe_sans -d "${BASE_DIR}/test/repo" -t "${TEMP_LOCAL}/" -B desktop1 install vim

     exe -d "${BASE_DIR}/test/repo" -t "${TEMP_LOCAL}/" --hostname desktop1 --no uninstall vim

     # make sure it exited ok
     local last="$?"
     [[ "$last" != "0" ]] && return $last

     # check the linked files exist
     assert_link "${TEMP_LOCAL}/.vimrc" "${BASE_DIR}/test/repo/vim/hosts/desktop1/files/.vimrc" || return 1
     assert_link "${TEMP_LOCAL}/.vim/filetype.vim" "${BASE_DIR}/test/repo/vim/files/.vim/filetype.vim" || return 1
     assert_link "${TEMP_LOCAL}/.config/i3/config" "${BASE_DIR}/test/repo/vim/hosts/desktop1/files/.config/i3/config" || return 1

     return 0
}

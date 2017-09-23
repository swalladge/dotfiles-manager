#!/bin/bash


run_test() {
     echo "checking that adding a file to a package works"

     # make some files/dirs
     echo "set compatible" > "${TEMP_LOCAL}/.vimrc2"
     echo "# hi" > "${TEMP_LOCAL}/.zshrc"

     # copy the repo
     cp -r "${BASE_DIR}/test/repo" "${TEMP_LOCAL}"
     ls -a "${TEMP_LOCAL}"

     echo "no" | exe -d "${TEMP_LOCAL}/repo" -t "${TEMP_LOCAL}/" -B desktop1 add "${TEMP_LOCAL}/.vimrc2" -p vim

     # make sure it exited ok
     local last="$?"
     [[ "$last" != "0" ]] && return $last

     # link should not be created!
     assert_link "${TEMP_LOCAL}/.vimrc2" "${TEMP_LOCAL}/repo/vim/files/.vimrc2" && return 1
     assert "file should not exist in repo" ! -e "${TEMP_LOCAL}/repo/vim/files/.vimrc2" || return 1

     return 0
}

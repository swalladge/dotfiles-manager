#!/bin/bash


run_test() {
     echo "checking that adding a file to a package works (host specific mode) even when repo directories don't exist"

     # make some files/dirs
     echo "set compatible" > "${TEMP_LOCAL}/.vimrc2"
     echo "# hi" > "${TEMP_LOCAL}/.zshrc"

     # copy the repo to the local directory so we don't mess up the original
     cp -r "${BASE_DIR}/test/repo" "${TEMP_LOCAL}"

     exe -d "${TEMP_LOCAL}/repo" -t "${TEMP_LOCAL}/" -B desktop2 -y add "${TEMP_LOCAL}/.vimrc2" -p vim --host

     # make sure it exited ok
     local last="$?"
     [[ "$last" != "0" ]] && return $last

     assert_link "${TEMP_LOCAL}/.vimrc2" "${TEMP_LOCAL}/repo/vim/hosts/desktop2/files/.vimrc2" || return 1

     return 0
}

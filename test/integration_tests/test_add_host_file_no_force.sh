#!/bin/bash


run_test() {
     echo "checking that adding a file to a package works (host specific mode)"
     echo "also in force mode"

     # make some files/dirs
     echo "set compatible" > "${TEMP_LOCAL}/.vimrc2"
     echo "# hi" > "${TEMP_LOCAL}/.zshrc"

     # copy the repo to the local directory so we don't mess up the original
     cp -r "${BASE_DIR}/test/repo" "${TEMP_LOCAL}"
     echo "hi" > "${TEMP_LOCAL}/repo/vim/hosts/desktop1/files/.vimrc2"

     exe -d "${TEMP_LOCAL}/repo" -t "${TEMP_LOCAL}/" -B desktop1 add "${TEMP_LOCAL}/.vimrc2" -p vim --host

     # make sure it exited with fail status (should not have overwritten)
     local last="$?"
     [[ "$last" == "0" ]] && return 1

     assert ".vimrc2 should not be modified" "$(cat "${TEMP_LOCAL}/repo/vim/hosts/desktop1/files/.vimrc2")" = "hi" || return 1

     return 0
}

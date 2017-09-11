#!/bin/bash


run_test() {
     echo "checking that a non-forced install does not overwrite existing files"

     # make some files/dirs
     echo "set compatible" > "${TEMP_LOCAL}/.vimrc"
     mkdir -p "${TEMP_LOCAL}/.vim/filetype.vim"
     local original_dir=$(readlink -f "${TEMP_LOCAL}/.vim/filetype.vim")

     # run without force on
     exe -d "${BASE_DIR}/test/repo" -t "${TEMP_LOCAL}/" -B desktop1 install vim

     # should not have existed ok
     local last="$?"
     [[ "$last" == "0" ]] && return 1

     assert ".vimrc should not be modified" "$(cat "${TEMP_LOCAL}/.vimrc")" = "set compatible" || return 1

     readlink "${TEMP_LOCAL}/.vim/filetype.vim" && { echo "filetype.vim overwritten!"; return 1; }

     assert "filetype.vim should not be modified" "$(readlink -f "${TEMP_LOCAL}/.vim/filetype.vim")" = "$original_dir" || return 1

     return 0
}

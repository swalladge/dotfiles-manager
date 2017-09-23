#!/bin/bash


run_test() {
     echo "checking that uninstalling a package with existing non-package symlinks will not forcibly remove them"

     # make some files/dirs
     echo "set compatible" > "${TEMP_LOCAL}/.vimrc"
     echo "# hi" > "${TEMP_LOCAL}/.zshrc"
     mkdir -p "${TEMP_LOCAL}/.vim/filetype.vim" # warning, this is a dir for testing

     exe -d "${BASE_DIR}/test/repo" -t "${TEMP_LOCAL}/" -B desktop1 --yes remove vim zsh

     # make sure it exited ok
     local last="$?"
     [[ "$last" != "0" ]] && return $last

     # check the linked files don't exist any more
     assert "vimrc should not be removed" -e "${TEMP_LOCAL}/.vimrc" || return 1
     assert "zshrc should not be removed" -e "${TEMP_LOCAL}/.zshrc" || return 1
     assert "filetype.vim should not be removed" -e "${TEMP_LOCAL}/.vim/filetype.vim" || return 1
     assert ".vim/ directory should not be removed" -d "${TEMP_LOCAL}/.vim/" || return 1

     return 0
}

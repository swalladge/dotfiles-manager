#!/usr/bin/env bash


# http://www.ostricher.com/2014/10/the-right-way-to-get-the-directory-of-a-bash-script/
get_script_dir () {
     SOURCE="${BASH_SOURCE[0]}"
     while [ -h "$SOURCE" ]; do
          DIR="$( cd -P "$( dirname "$SOURCE" )" && pwd )"
          SOURCE="$( readlink "$SOURCE" )"
          [[ $SOURCE != /* ]] && SOURCE="$DIR/$SOURCE"
     done
     $( cd -P "$( dirname "$SOURCE" )" )
     pwd
}

export BASE_DIR="$(get_script_dir)"
cd "$BASE_DIR"


# temporary local in-repo directory
export TEMP_LOCAL="${BASE_DIR}/local/"
rm -rf "$TEMP_LOCAL"
export TEMP_HOME="${BASE_DIR}/local/home/"
mkdir -p "$TEMP_HOME"


export BIN="cargo run --bin dotfiles-manager -- "

export TESTS_BASE_DIR="${BASE_DIR}/test/integration_tests"
export TESTS_DIR="${TESTS_BASE_DIR}/tests"

ok=0
for filename in ${TESTS_DIR}/*; do
     source "${filename}"
     LAST=$?
     [ "$LAST" == "0" ] || ok="$LAST"
done

exit "$LAST"

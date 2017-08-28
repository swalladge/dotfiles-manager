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
cd "$BASE_DIR" || exit 1

[[ -n "$KCOV_BIN" ]] || KCOV_BIN=kcov

# temporary local directory
export TEMP_LOCAL="${BASE_DIR}/local/"

# use this to run the executable
# generates code coverage data with every run
# slow but comprehensive
exe() {
     local coverage_dir="target/cov/$(uuidgen)"
     mkdir -p "$coverage_dir"
     $KCOV_BIN --exclude-pattern=/.cargo,/usr/lib --verify "$coverage_dir" "${BASE_DIR}/target/debug/dotfiles-manager" "$@"
}

echo ":: Setup complete, begin tests."

count=0
TESTS_DIR="${BASE_DIR}/test/integration_tests"
for filename in ${TESTS_DIR}/*; do

     rm -rf "$TEMP_LOCAL"
     mkdir -p "$TEMP_LOCAL"

     # each test file should be a bash script with no global variables,
     # defining a `run_test` function
     # variables to use:
     # - exe          | the binary to run for the dotfiles manager
     # - TEMP_LOCAL   | the local directory to do stuff in - make files, etc - reset after each test
     # - BASE_DIR     | root directory of project
     echo ""
     echo ":: Running test $(basename $filename)"
     source "${filename}"
     run_test

     # check the return value - if non-zero, we want to exit with that value
     # but still run the rest of the tests
     LAST=$?
     if [ "$LAST" != "0" ]; then
          echo ""
          echo ":: Test failed with exit code ${LAST}"
          ((count++))
     fi
done

if [ "$count" != "0" ]; then
     [ "$count" != "1" ] && plural="s"
     echo ""
     echo ":: ${count} test${plural} failed!"
else
     echo ""
     echo ":: All tests successful!"
fi

exit "$LAST"

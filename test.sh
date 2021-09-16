#!/usr/bin/env sh

testCargo() {
  NO_COLOR=1 cargo test --lib > cargo-test.log 2> cargo-test.log
  assertEquals "0" "$?"
}

assertFile() {
  expected_file="$1"
  expected_content="$2"

  assertTrue "[ -f $expected_file ]"
  assertEquals \
    "$expected_content" \
    "$(cat --show-ends "$expected_file" | tr -d '\r\n')"
}

testOneLine() {
  assertFile \
    "target/tests/create-file-macro/one-line.expect.txt" \
    "foo bar baz"
}

testMultipleLines() {
  assertFile \
    "target/tests/create-file-macro/multiple-lines.expect.txt" \
    "foo\$bar\$baz"
}

testMultipleLinesEndNewline() {
  assertFile \
    "target/tests/create-file-macro/multiple-lines-end-newline.expect.txt" \
    "foo\$bar\$baz\$"
}

testDontOverwriteMe() {
  assertFile \
    "target/tests/dont-overwrite-me.expect.txt" \
    "This file shouldn't be overwritten!\$"
}

testRelativePath() {
  assertFile \
    "target/tests/relative-path.expect.txt" \
    "foo bar baz qux"
}

testPathToSubdirectory() {
  assertFile \
    "target/tests/create-file-macro/foo/path-to-subdirectory.expect.txt" \
    "Hello from subdirectory!"
}

oneTimeSetUp() {
  # create file before 'cargo test' which shouldn't be overwritten by the macro
  if [ ! -d "target/tests" ]; then
    mkdir -p target/tests
  fi

  # (note that is created outside of the target/tests/create-file-macro folder)
  echo "This file shouldn't be overwritten!" \
    > target/tests/dont-overwrite-me.expect.txt
}

prepare() {
  if [ ! -f "shunit2" ]; then
    curl -sSL https://raw.githubusercontent.com/kward/shunit2/master/shunit2 \
      -o shunit2
  fi

  # rebuild target/tests crate
  rm -rf target/tests
}

prepare && . ./shunit2

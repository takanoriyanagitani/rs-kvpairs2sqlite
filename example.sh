#!/bin/bash

wsm="./kvpairs2sqlite.wasm"

hdir="$PWD"
gdir="/guest.d"
bname="example.sqlite.db3"
gpath="${gdir}/${bname}"
hpath="${hdir}/${bname}"

run_wasi(){
  cat /dev/stdin |
    wasmtime \
      run \
      --env ENV_SQLITE_DB_FILE_PATH="${gpath}" \
      --dir "${hdir}::${gdir}" \
      "${wsm}"
}

input1(){
  jq -c -n '["helo", "wrld"]'
  jq -c -n '["hello", "world"]'
}

echo -- writing to sqlite... --
input1 | run_wasi
echo

echo -- printing the content... --
sqlite3 \
  -markdown \
  "${hpath}" \
  "
    SELECT * FROM kvpairs
  "

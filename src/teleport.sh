#!/bin/sh

function tp() {
  result=$(teleport-rs $@)
  rc=$?

  if [ $rc -eq 2 ]
  then
    cd "$result"
  elif [ -z "$result" ]
  then
    echo -n
    return $rc
  else
    echo "$result"
    return $rc
  fi
}

#!/usr/bin/env bash
for file in $(git ls-files); do
  echo $file
  echo
  cat $file
  echo
done

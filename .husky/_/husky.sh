#!/usr/bin/env sh
if [ -z "$husky_skip_init" ]; then
  husky_dir=$(dirname "$0")/..
  export PATH="$husky_dir/node_modules/.bin:$PATH"
  if [ -f "$husky_dir/.huskyrc" ]; then
    . "$husky_dir/.huskyrc"
  fi
  husky_skip_init=1
  sh -e "$husky_dir/$(basename "$0")" "$@"
  exit $?
fi

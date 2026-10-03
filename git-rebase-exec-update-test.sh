#!/bin/zsh

set -euo pipefail

local -r test_dir="tests/tests/swfs/avm2/displayobject_invalid_floats"

. ~/dev/flashworkspace/.zshrc.ruffle

git log -1 --name-only --pretty='' | grep -Fq "$test_dir/Test.as" || { echo "no change found; skipping"; exit 0 }

local -r dir="$test_dir"

if [ -d "$dir" ]; then
    cd "$dir"

    mxmlc -debug Test.as -o test.swf && fpdwine test.swf && mv flashlog.txt output.txt -f

    cd -
fi


git add .
git commit --amend --no-edit

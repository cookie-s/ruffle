#!/bin/zsh

set -euo pipefail

. ~/dev/flashworkspace/.zshrc.ruffle

drop_fail_git_commit_message() {
    for i in $(seq 10)
    do
        msg="$(git log -1 --pretty='%B')"
        newmsg=$(echo "$msg" | sed '1s/![0-9][0-9]* //')
        git commit -m "$newmsg" --amend
    done
}

leave_fail_git_commit_message() {
    local -r idx="$1"
    git commit -m "!${idx} $(git log -1 --pretty='%B')" --amend
}

drop_fail_git_commit_message

cargo test -p ruffle_core -- test_all_nan || \
    { leave_fail_git_commit_message 10 ; }

cargo test -p ruffle_core -- test_no_skew || \
    { leave_fail_git_commit_message 11 ; }

cargo test -p ruffle_core -- test_assign_nanmat_assign_rot0_prev_rot || \
    { leave_fail_git_commit_message 12 ; }

cargo test -p ruffle_core -- test_assign_nanmat_assign_rot0_x || \
    { leave_fail_git_commit_message 13 ; }

cargo test -p ruffle_core -- test_assign_nanmat_assign_rot0_y || \
    { leave_fail_git_commit_message 14 ; }

cargo test -p ruffle_core -- test_assign_rotnan || \
    { leave_fail_git_commit_message 15 ; }

cargo test -p tests -- invalid_floats || \
    { leave_fail_git_commit_message 2 ; }

cargo test -p tests -- mat_to_transform || \
    { leave_fail_git_commit_message 3 ; }

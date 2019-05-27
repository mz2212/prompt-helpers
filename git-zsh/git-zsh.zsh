#!/bin/zsh

function _git_status() {
	_dir_status="$(git status --porcelain 2> /dev/null)"
	if [ $? -eq 0 ]; then
		if [ "$_dir_status" = '' ]; then
			echo "%B%F{green}✓%b%f "
		else
			echo "%B%F{red}✗%b%f "
		fi
	fi
	unset _dir_status
}
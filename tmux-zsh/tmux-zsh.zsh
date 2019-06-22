#!/bin/zsh

function _tmux_detached_count() {
	if [ -v $TMUX ]; then
		_tmux_lines="$(tmux ls 2> /dev/null)"
		if [ $? -eq 0 ]; then
			_lines_count="$(echo $_tmux_lines | wc -l)"
			echo "%B%F{blue}t$_lines_count%b%f "
			unset _lines_count
		fi
		unset _tmux_lines
	fi
}

_cerberusctl()
{
    export CERBERUSCTL_COMPLETION_CACHE
    local cur prev cmds base

    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"

    if [ -z "$CERBERUSCTL_COMPLETION_CACHE" ]; then
        help_output=$(cerberusctl --help | grep '^  [a-z]' | awk '{ print $1 }')
        export CERBERUSCTL_COMPLETION_CACHE="$help_output"
    fi

    cmds="$CERBERUSCTL_COMPLETION_CACHE"

    COMPREPLY=($(compgen -W "${cmds}" -- ${cur}))
    return 0
}

complete -F _cerberusctl cerberusctl

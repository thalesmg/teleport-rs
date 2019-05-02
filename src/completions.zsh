#compdef tp
# -*- mode: sh-mode; -*-

_tp() {
  local line

  _arguments -C \
     "1: :(go list rm add)" \
     "*::arg:->args"

  case $line[1] in
    go|rm)
      _list_portals
      ;;
    *)
      ;;
  esac
}

__mkline() {
  local line key value
  while IFS=$'\t' read -A line; do
    key=$(echo "${line[1]}" | tr -d '[:blank:]')
    value=$(echo "${line[2]}" | tr -d '[:blank:]')
    echo "$key\\:$value"
  done
}

_list_portals() {
  local portals
  portals=($(tp list | sed -e '1d' | __mkline))

  _arguments \
    ":::(($portals))"
}

_tp "$@"

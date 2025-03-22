_:
  @just --list

run:
  cargo run --bin bg-serv

_new-window NAME CMD:
  tmux new-w -t wallpaper -n "{{NAME}}"
  tmux send-keys -t wallpaper:"{{NAME}}" "{{CMD}}" ENTER

tmux:
  tmux new -ds wallpaper -n "README"
  tmux send-keys -t wallpaper:README 'nv ./README.md "+set wrap"' ENTER
  @just _new-window "Edit" ""
  @just _new-window "Run" ""
  @just _new-window "Git" "git status"
  @just _new-window "Misc" ""
  tmux a -t wallpaper

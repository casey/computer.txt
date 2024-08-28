watch +args='test':
  cargo watch --clear --exec '{{args}}'

console:
  watchexec \
    --clear=clear \
    --filter computer.txt \
    --no-vcs-ignore \
    --quiet \
    cat computer.txt

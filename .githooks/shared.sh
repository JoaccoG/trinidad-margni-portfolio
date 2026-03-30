RED=$(printf '\033[0;31m')
GRAY=$(printf '\033[1;30m')
GREEN=$(printf '\033[0;32m')
YELLOW=$(printf '\033[1;33m')
RESET=$(printf '\033[0m')

hook_start() {
  printf "\n${YELLOW}(>)${RESET} ${GRAY}Running '${RESET}${YELLOW}${1}${RESET}${GRAY}' hook...${RESET}\n"
}

hook_done() {
  printf "${GREEN}(✔)${RESET} ${GRAY}Successfully executed '${RESET}${GREEN}${1}${RESET}${GRAY}' hook.${RESET}\n"
}

step() {
  printf "${YELLOW}(→)${RESET} ${GRAY}${1}${RESET}\n"
}

fail() {
  printf "${RED}(!)${RESET} ${GRAY}${1}${RESET}\n"
}

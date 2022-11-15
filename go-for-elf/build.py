# bin python3
# python3 build.py

import os

name = "elf"
path = "./bin"

go_mod_tidy = 'go mod tidy'
os.popen(go_mod_tidy)

go_build = 'go build -ldflags "-s -w" -o {}/{} main.go'.format(path, name)
os.popen(go_build)

go_upx = 'upx -9 ./bin/*'
os.popen(go_upx)

alias b := build

server_dir := "server"

web_dir := "web"
web_dist := "web" / "dist"

bun := `which bun`
cargo := `which cargo`

default: run

check: check-server check-web
build: build-server build-web
test: test-server test-web
clean: clean-server clean-web
fmt: fmt-server fmt-web

all-server: check-server build-server test-server
all-web: check-web build-web test-web

# Server

run: build-web
	cd {{server_dir}} && {{cargo}} run --release

run-dev: build-web
	cd {{server_dir}} && {{cargo}} run

check-server:
	cd {{server_dir}} && {{cargo}} check

build-server:
	cd {{server_dir}} && {{cargo}} build --release

test-server:
	cd {{server_dir}} && {{cargo}} test

clean-server:
	cd {{server_dir}} && {{cargo}} clean

fmt-server:
	cd {{server_dir}} && {{cargo}} fmt

# Web

check-web:
    cd {{web_dir}} && {{bun}} x astro check

build-web:
	cd {{web_dir}} && {{bun}} run build

test-web:
	cd {{web_dir}} && {{bun}} test

clean-web:
	rm -rf {{web_dist}}

fmt-web:
	cd {{web_dir}} && {{bun}} x prettier --write .

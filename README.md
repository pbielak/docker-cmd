# docker-cmd
[![crates.io](http://meritbadge.herokuapp.com/docker-cmd)](https://crates.io/crates/docker-cmd) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

A simple CLI tool for running commands in Docker containers.

### Motivation
You could ask yourself, what's the point of this tool. While working with Docker containers, 
I often encountered the following problem: I wanted to launch a new bash shell inside an existing 
container, I started to type `docker exec -it` and then I realised that I need to pass some
info about the target container (either the ID or the container name). In such situation I launched
another terminal, executed the `docker ps` command, looked for the appropriate container,
copied its ID and pasted it into the original `docker exec` command. This was a lot of 
repetitive and frustrating work. 

I decided to make my life easier, so I've written this tool. I hope you'll like it. In case of any
problems and/or feature requests, please create an issue on GitHub. 

### Quickstart
Run `docker-cmd`:
```bash
$ docker-cmd
   ID            IMAGE          COMMAND    STATUS
 > 672477a625ac  ubuntu:xenial  /bin/bash  Up 2 minutes
   eb73cf7eaec8  python:3.6     python3    Up 20 minutes
```
Select the desired container (arrow keys) and press Enter:
```bash
root@672477a625ac:/# 
```
The default executed command is `/bin/bash`, but you can pass any command to execute (see help).

Note that if there is only one container, the tool will automatically use this container 
and launch the desired command in it.

### Changelog
* v0.3
   - Added function for locating Docker binary, instead of hardcoded value
   - Applied clippy hint (unnecessary string cloning)

# docker-cmd
[![crates.io](http://meritbadge.herokuapp.com/docker-cmd)](https://crates.io/crates/docker-cmd) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

A simple CLI tool for running commands in Docker containers.

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

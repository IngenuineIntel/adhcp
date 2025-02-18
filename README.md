# adhcp

TUI for administrating an ISC-DHCP server on Linux, written in Rust.

NOT DONE
---
this README reflects the same unfinished state as the entirety of the
project

## Why?

Using the ISC-DHCP server on Linux is a cheap alternative to Windows
Server's DHCP server, but the solutions for administrating them are
tedious and not very intuitive long term. As such, I found myself
wanting such a program as this to exist. I wrote it in Rust because
Python would be very resource intensive (which doesn't fit into a
server context well) and because C just isn't the tool for the job.
Also, I needed an excuse to get used to Rust, and Rust is most likely
to be contributed to, it seems.

## Running

This program only runs on servers with `dhcpd` running (and only on
systems with `journalctl` (therefore only servers that use
`systemd`)), so be sure to either be on your server or SSHed into it.

Then write something like this:

```bash
~$ git clone https://github.com/ingenuineintel/adhcp
~$ cd adhcp
~$ cargo build
~$ sudo cp ./target/debug/adhcp /usr/local/bin
```

## Usage

Getting there.


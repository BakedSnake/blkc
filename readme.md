## Blkc :: A tool to run commands across a list of servers

#### Build:

```
$ cargo build --release
```

( requires cargo )

#### Install:

```
$ make config
$ sudo make install
```

#### Dependencies:
This program is meant to be run on a linux / unix system that has `ssh` and `pass` installed and configured.

It is also assumed that you have ssh keys setup on your servers.
On the file `~/.config/blkc/blkc.conf` you set which key to use.

#### Usage

Copy the template file into `~/.config/blkc/list.json`. ( This is the file that the program "looks for". ).

Add servers to the list on the list file:

```
[
  {
    "id": 0,
    "label": "Server label for group commands",
    "name": "Server Name",
    "user": "User Name",
    "address": "IP Address or WAN",
    "sshport": "SSH connection port"
  },
  {
    "id": 1,
    "label": "Example",
    "name": "Example-Name",
    "user": "example-user",
    "address": "0.0.0.0",
    "sshport": "22"
  },
  {
    "id": -1,
    "label": "Label not found",
    "name": "No server with that name was found.",
    "user": "No user found.",
    "address": "No ",
    "sshport": "Could not use port"
  }
]
```

Please note that all new server entries should go in between the entries with id `0` and `-1`. Otherwise issues may occur.

##### Run command on a single server:

```
$ blkc -r -n < server name > "< command >"
```

##### Run command on multiple servers:

```
$ blkc -r -l < server label > "< command >"
```

This will run the command on all the servers that have the same label.

##### Run command as root

```
$ blkc -sr -n < server name > "< command >"
```

`sudo` should not be used in the command.

#### Uninstall:

```
blkc/ $ make clean
```

#### TODO:
Better error handling.
Proper ssh implementation.
Improve authentication.

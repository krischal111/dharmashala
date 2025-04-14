# Notes for Linux Elective
## Theory

The contents here are directly sourced and paraphrased from Mr. Raju Musyaju materials, with focus on exams, and covering only the required content.

For supplementary contents which provide clarification for some of the contents, refer [More notes](More-Notes.md)

### Chapter 1: Introduction to Linux [2 Hrs]

**Multics (MULTiplexed Information and Computing Service)**
- Considered as the origin of Unix and Linux
- Developed by MIT, Bell labs and General Electric
- Time sharing OS
- Based on *Single-level memory*
- Hierarchical Filesystem (No direct access to FS)
- Designed to run on GE 645 mainframe

Reference:
- <https://en.wikipedia.org/wiki/Multics>

**Linux:**
- Developer: Linus Torvalds
- License: GNU GPL
- Inspired by: MINIX
- Based on: GNU project
- Open Source OS
- Free
- Stable and Secure
- Cross Platform

### Chapter 2. Install Linux and tools [6 Hrs]

- Rocky linux can be downloaded from <https://rockylinux.org/download>
- It's available in 3 versions:
    - DVD ISO:
        - Full installation media
        - Contains OS + packages + GUI
        - Offline Installability
        - Use if you want to complete install offline, or you are not sure what packages you need.
        - Good for Desktop
    -  Minimal ISO:
        - Enough to install a basic system
        -  NO GUIs and no extra packages
        - Can be installed offline
        - Use if you want to create minimal servers, containers, VMs, or if you *have*/*are using* post install setup scripts.
        - Good for Custom servers
    - Boot ISO:
        - Very minimal installer
        - No extra packages
        - Needs internet. Anything you need to install will be downloaded from the internet.
        - Use if you want to do network install where you choose packages from mirror or your own repo, and you have fast stable internet connection
        - Good for enterprise setup or PXE environments.
- It's also available for the following platforms:
    - AMD/Intel (x86_64)
    - ARM (aarch64)
    - PowerPC (pp64le)
    - IBM Z (s390x)

#### Rocky linux installation on VM

Steps

1. Install a suitable virtualization softwares like (any one):
    - Virtualbox
    - Parallels
    - UTM
    - VMware workstations

2. Download Rocky Linux Mimimal ISO from <https://rockylinux.org/download>

3. Create a virtual machine with Network set to "Bridge Adapter" mode

4. During install select "Allow root SSH login with password" (not recommended for production).

5. Proceed with other steps normally.

#### SSH tools installation

For windows, download puttygen for public key generation and putty for accessing SSH servers.

For linux, openssh is installed by default which can be used for accessing SSH servers. Likewise, use ssh-keygen to generate public key pairs.

Instructions can be found on the internet.

Example command to create ssh key:

    ssh-keygen -t ed25519 -C "your_email@example.com"

If you want to provide the file path for ssh key, and also want to give the password:

    mkdir -p parent_dir # parent_dir is usually ".ssh"
    ssh-keygen -t ed25519 -C "your_email@example.com" -f "parent_dir/identity_file" -N "password"

To copy the ssh key to the new node (server):

    ssh-copy-id -i "identity_file" username@IP_address_of_server

To access the new node as the user

    ssh -i "identity_file" username@IP_address_of_server


### Chapter 3. Access Linux [6 Hrs]

Excellent introduction to UNIX anatomy (similar to linux): <http://ibgwww.colorado.edu/~lessem/psyc5112/usail/concepts/anatomy-of-unix/anatomy.html>

![UNIX anatomy](http://ibgwww.colorado.edu/~lessem/psyc5112/usail/concepts/images/anatomy.gif)

#### Shell

- A shell provides a command line.
- It acts as intermediary between kernel and an user.
- Default shell in most linux (like rocky linux) is GNU Bourne-Again Shell (bash).
- Root user's prompt with a `#`.
    ```
    [root@rocky-linux ~]# _
    ```
- Normal user's prompt start with a `$`.
    ```
    [lisa@rocky-linux ~]$ _
    ```

Some common shells are:
- csh: C like shell
- sh: Posix standard shell
- tcsh: Improved shell
- zsh: Improved bourne shell
- fish: Friendly interactive shell
- nushell: A structured shell with support for data types and functional approach.

#### Shell working loop

The loop of the shell can be summarized in following steps:

    Display the prompt.
    Read the command.
    Analyze the syntax.
    Substitute special characters
    Execute the command

    Display the prompt
    ...
    and so on

#### Command Structure

Command may contain options and arguments.
- Options 
    - Option modify behavior of the command.
    - Option preceed with a dash (`-`, mostly for single character) or a double dash (`--`, mostly for a word).
    - Example: `ls -a` or `ls --all`
    - Some commands allows combining multiple single character options into a single one. Example `ls -a -l` or `ls -al` mean same thing.
- Arguments
    - Arguments are the input that the commadn will act upon
    - Example: `cat -n /etc/hosts`

Essential linux commands:

1. `cd`: Change directory
2. `ls`: List files and directories.
3. `ln`: Create symbolic links.
4. `pwd`: Print working directory.
5. `cp`: Copy
6. `mv`: Move
7. `scp`: Securely copy files.
8. `chown`: Change ownership of a file.
9. `chmod`: Change file access permissions.
10. `lsattr`: List attributes
11. `chattr`: Change attributes


#### Man page

- `man` pages can be used to get usage of any commands.
- `man` pages are indexed in `mandb`
- `man` page is divided into 8 sections, can be remembered by the following sentence: Good Students Learn Keenly For Great Marks, Sometimes Knowing.
    - General Commands Manual (default)
    - System Calls Manual
    - Library Function Manual
    - Kernel Interfaces Manual
    - File Formats Manual
    - Games Manual
    - Miscellaneous Information Manual
    - System Manager's Manual
    - Kernel Developer's Manual

- To find the right man page:
`man -a passwd`
`man -k passwd` or `apropos passwd`

#### OS specific

- To see the version of the rocky linux
```
    cat /etc/rocky-release
```

- To see the information about the system
```
    uname -a
```
Options of unames
```
Options:
    -s : Kernel's name
    -n : Nodename (name of the system)
    -v : Kernel's version
    -o : Operating system
    -r : Kernel release
```

- To see the user id of an user:
```
    id -u username
```
Output: Uid of the user named `username`.

Example: Output of `id -u root` will always be `0`

- If you omit the username, you will get the uid of current user.
```
    id -u
```
Output: Uid of the current user.


#### Super user (root)

- The user called root has the highest level of right or access capability.
- Avoid accessing the system as the root user unless necessary.
- Root user's UID is 0.

#### sudo vs su

- `sudo` was originally meant to be a command that allowed any user to execute a super user's command, thus its name orinally stood for **super user do**.
- However, later version of `sudo` also allowed to execute commands as other users, making its new full form **substitue user do**.
- With `sudo -i` we can even open interactive shell for root users.
- With `sudo -u username command` we can launch command for any user `username`.
- Most likely, `sudo` will ask for your password. (The user currently logged in.)
- Sudo may not ask password in following conditions:
    - Timeout for previously entered password is not reached.
    - In sudoers file, an user is allwoed to run sudo without password with `:NOPASSWD`

- `su` means switch user (or substitue user)
- It allows us to log in as another user without closing current user.
- With `su username -c command` we can execute a command as the user `username`
- `su` will always ask for the password of the user you are switching as. (Unlike sudo which asks the password of current user.)


Example:
```
    [ramesh@rocky-linux ~]$ su ram -c ls
```
The above command will ask the password of `ram`, and then execute the command as `ram`.

```
    [ramesh@rocky-linux ~]$ sudo -u ram ls
```
The above command will ask the password of `ramesh`, and then execute the command as `ram`.


#### sudoers file

The file `/etc/sudoers` is known as the sudoers file. It can be accessed by the command:
```
    visudo
```

Structure of sudoers file:

The structure of sudoers file is:

```sudoers
user hostlist = (userlist) commandlist
```
Meaning: An user `user` logged in from one of the host from `hostlist` can run any command from `commandlist` as any user form the `userlist`.

If group instead of user is to be used, preceed it with a `%` as in
```
%group hostlist = (userlist) commandlist
```

Examples

```
# Ram can execute any command as any user, but only from rocky-peeps host
ram rocky-peeps = (ALL) ALL
```

```
# Ram can execute any command from any host but only as user lisa
ram ALL = (lisa) ALL
```

```
# Ram can execute only `/usr/bin/ls` and `/usr/bin/cat` command but as any user and from any host
ram ALL = (ALL) /usr/bin/ls, /usr/bin/cat
```

```
# Ram can execute any command as any user and from any host
ram ALL = (ALL) ALL
```

```
# Ram can execute any command as any user and from any host and no password is prompted when doing so
ram ALL = (ALL) NOPASSWD: ALL
```

When an user uses sudo to attempt something he cannot do, it may say "This incident will be reported." To see the report, you can simply observe the file `/var/log/secure` as:

```
    less -f /var/log/secure
```
        



### Chapter 4. Introduction to File System [4 Hrs (Assumed)]

### Chapter 5. File manipulation [4 Hrs]

### Chapter 6. Mastering Vim [6 Hrs]

### Chapter 7. Users/Groups and File Permissions Core Information [4 Hrs]

### Chapter 8. Basic networking [6 Hrs]

### Chapter 9. Manage Basic Storage [10]

### Chapter 10. Advanced Commands (Regular Expressions) [6 Hrs]

### Chapter 11. Basic of Systemd [6 Hrs]

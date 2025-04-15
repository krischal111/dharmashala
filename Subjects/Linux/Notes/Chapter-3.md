### Chapter 3. Access Linux [6 Hrs]

Excellent introduction to UNIX anatomy (similar to linux): <http://ibgwww.colorado.edu/~lessem/psyc5112/usail/concepts/anatomy-of-unix/anatomy.html> (If it doesn't work, make sure your browser didn't substitute http for https. Copy pasting may work.)

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
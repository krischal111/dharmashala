### Chapter 7. Users/Groups and File Permissions Core Information [4 Hrs]

#### User:
- An individual or entity that interactss with the OS
- Each user has:
    - A unique username
    - Unique user id (uid)
- OS keeps trask of user's identity and permissions to access system resources
- Users information is stored in /etc/passwd file with passwords on /etc/shadow file.

#### Type of users
- Superuser (root): Root is the superuser with UID of 0.
- System user: Users to run system services and not meant for human interactions. Also called service accounts. They generally have UID below 1000 (though this varies by distro). They have no password, and login shell is disabled. Examples:
    - daemon
    - sys
    - nobody
    - sshd
    etc.
- Regular user: Human users who have access to the system.


User related option file: /etc/login.defs

useradd: add new user   -> create home dir default if CREATE_HOME=yes
usermod: modify user
-c : comment
-e : expiredate
-g : primary group
-G : secondary group
-k : skel directory
-m : create home directory 
-M : do not create home directory
-p : password
-r : system account
-s : shell    eg. -s /usr/bin/zsh
-u : user id

Lock User : usermod -L username
Change Password : echo "Password" | passwd <username> --stdin
Lock Password: passwd -S <username>

List of Users: getent passwd     OR     cat /etc/passwd
Look user info : getent passwd <username>      OR       cat /etc/passwd | grep "^<username>"
Look user password info : getent shadow <username>   ->   ! shown if password not set
Look user group : groups <username>
Look user password age: chage -l <username>
userdel: remove  user
-r : remove home directory too   eg. sudo userdel -r binit

#### Groups
- Helps to organize a collection of users.
- Associated with unique ID called Group ID (gid)
- Two types:
    - Primary group : Each user is a member of a primary group.
    - Supplementary group : Each user is a member of zero or more supplementary group.
- Group insformation is stored in /etc/group and respective passwords is stored in /etc/gshadow file

#### Creation and modification of Users and Group

Commands:
**useradd**:
```
    useradd [options] username
```
- Sets up a user's home directory
- Also creates aprivate group designated by user's username
- No password is set yet

**passwd**:
```
    passwd username
```
- Adds password to the users

To change the password without using script:
```
    echo 'new_password' | passwd username --stdin
```


**getent**:
```
    getent passwd username
```
- Get information about the user.

```
    getent shadow username
```
- Get password information about user.
- !! indicates no password is set

**chage**:
```
    chage -l username
```
- Displays the password expiry information about the user


**usermod**:
```
    usermod -g groupname username
```
- Adds an user to the primary group
```
    usermod -G groupname username
```
- Adds an user to the secondary group

```
    usermod -L mytest
```
- Locks the user



**groupadd**:
```
    groupadd electronics -g 10 -U lisa
```
- Adds a group named electronics with gid 10 and adds lisa to it.

```
    groups lisa
```
- List the gropus lisa belongs to


#### File permissions

file permission looks like:
`drwxr-xr--` 

This can be separated into 4 parts: `d` `rwx` `r-x` `r--`.

First part represents the type of file. (discussed in chapter 5)

Second part represents the owner's permission.

Third part represents the group's permission.

Fourth part represents the other's permission.

**Permissions:**
- r = read
- w = write
- x = eXecute or eXplore (file vs directories)

**Symbolic notation**
- rwx pattern can be substituted by bits depending on their presence or absence.
- rwx = 111 (binary) = 7 (octal)
- r-- = 100  = 4
- r-x = 101  = 5
- rw- = 110  = 6

Example
rwxr-xr-- becomes 754

Useful commands:

- `chmod`: Change file permissions. Options:
    - `-R`: Recursively (when applied to directory, this applies to all its contents too.)
    - `chmod 775 file` gives rwxrwxr-x permission to file.
    - `chomod ug+rwx,o+r file`: Adds rwx permission to u(ser) and g(roup), and only r permission to o(thers).
    - `chmod -x file`: Removes eXecute permission from the file for ugo (users, groups and others).
    - `chown user:group file`: Change file's owner user and owner group.
    - `chgrp group file`: change file's group owner.

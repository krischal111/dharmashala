### Chapter 4. Introduction to File System [4 Hrs (Assumed)]

- In Linux everything is a file.
- When file is opened, OS uses File System to load it from the underlying storage.
- Filesystem governs the file organization and access.
- Without FS, apps could access storage in incompatible ways (like: simultaneous write, reading and writing together) leading to cascade of errors (like: resource contention, data corruption and then data loss).
- The Filesystem Hierarchy Standard (FHS) defines the directory structure and contents.

Key directories in linux are:
- `/`: Root directory of entire Filesystem Hierarchy
- `/bin`: Essential command binaries (like: ls, cat, cp)
- `/sbin`: Essential system binaries (like: fsck, init, route)
- `/dev`: Device files (like: /dev/null, /dev/sda1, /dev/random)
- `/etc`: Host specific system-wide configuration files.
- `/home`: User home directories
- `/usr`: Secondary hierarchy for read only user data; this contains majority of multi-user applicaitons and utilities.
- `/var`: Variable data files like logs, spool etc.

References
- Wikipedia: <https://en.wikipedia.org/wiki/Filesystem_Hierarchy_Standard>
- FHS reference: <https://refspecs.linuxfoundation.org/FHS_3.0/fhs-3.0.html>

#### Filesystem Architecture
- Logical File System:
    - Facilitates open, close, and write actions on files.
- Virtual Filesystem:
    - Provides standardized interface, allowing different filesystems to coexist, and function.
- Physical Filesystem:
    - Responssible for space management, storage of data and retrieval.

#### Absolute and relative path

Absolute path is the location of file or directory specified from the root directory.
- example: /home/lisa/file.py

Relative path is the location of file or directory specified from the current directory.
- example if current directory is /home/lisa/, relative path is ./file.py

Useful commands
- `pwd`: Print working directory.
- `realpath`: Prints the absolute path given a relative path.


#### XFS vs ext4
XFS:
- 64-bit journaling file systems
- Better performance with large files.
- Maximum file size is 8EiB.
- Not compatible with ext FS.
- Capable of snapshot
- Excels at parallel I/O operations.

ext4:
- Journaling file system
- Better performance with small files
- Can store up to 16TB
- Backward compatible wit hext3 and ext2
- Better security features

Reference: <https://blog.purestorage.com/purely-educational/xfs-vs-ext4-which-linux-file-system-is-better/>

#### Utilities:

`fsck`: It refers to **FileSystem Consistency checK**.

Syntax:
```
    fsck [OPTIONS] path/to/device
```

Options:
- `-N`: Just print what would be done without doing anything.
- `-n`: Assume no to every action, so checks filesystem without correcting anything.

`blkid`: It is used to locate and print block device's attributes.
Syntax:
```
    blkid path/to/device
```

`lsblk`: List block devices.
Syntax
```
    lsblk [OPTIONS] [device]
```
if device is omitted, it shows all block devices.


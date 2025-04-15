### Chapter 5. File manipulation [4 Hrs]

#### File types
When you list file with `ls -l`, in the firs column you get something like:
```
drwxr-xr-x
```

First letter represents the type of the file.

| Character | File type                     | Command to create | Comment                     |
| --------- | ----------------------------- | ----------------- | --------------------------- |
|  -        | ordinary file                 | touch             |                             |
|  d        | directory                     | mkdir             |                             |
|  b/c      | device file (block/character) | mknod             | represents hdd/monitors etc.|
|  l        | symbolic link                 | ln                | mirror to other files       |
|  s        | socket files                  |                   | provide inter-process comm. |
|  p        | named pipe files              | mknodq            | allows processes to send data to other processes or receive data from other processes |

References:
- <https://www.computernetworkingnotes.com/linux-tutorials/different-types-of-files-in-linux.html#google_vignette>


#### Inode number

- Inode (Index Node) is a data structure in UNIX that describes file system object (files, directories).
- Each inode stores disk location and attributes of an object and its data. Attributes may include:
    - metadata (last change, access, modification)
    - owner (uid or group id)
    - permission
    - size
    - link count
    - number of block allcoated
    etc.

#### Useful commands

`file [OPTION] filename`:  Recognizes the type of data in a file.

`stat filename` : Display file status.

`find` : Search for files in a directory hierarchy.

Some examples of usage of find commands are:
```
# to list all regular files accessed 5 days ago inside any level inside of the current directory
find . -type f -atime +5
```

```
# find all files inside /tmp directory, ending with .txt, pass all that file names to tar -czvf back.tar.gz {} in one step.
find /tmp -iname "*.txt" -exec tar -czvf back.tar.gz {} \+
```

```
# Delete all files inside /tmp ending with .txt
find /tmp -iname "*.txt" -exec rm -f {}\
```

Remember `{}\` applies the command for each file one by one. `{}\+` applies the command for all files togeter as in.
for `a.py`, `b.c` command `-exec rm {}\+` becomes `rm a.py b.c` but for command `-exec rm {}\` it becomes `rm a.py` and `rm b.c` seperate.


References:
- <https://en.wikipedia.org/wiki/Inode>
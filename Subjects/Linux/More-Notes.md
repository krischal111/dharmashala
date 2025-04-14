# Supplementary Notes for Linux Elective
## Theory

The contents mentioned here are only to supplement the [main note](Notes.md)

### Chapter 1: Introduction to Linux [2 Hrs]

#### Multics
Multics was Time Sharing OS based on Single Level Memory. Those term's meaning were as follows.

#### Time Sharing OS:
In time sharing OS, concurrency sharing of computing resource is done by giving each user or a task a small slice of the processing time. The quick switch between users or the tasks gives an illusion of simultaneous execution. (https://en.wikipedia.org/wiki/Time-sharing)

#### Single Level Memory
In context of Multics, *Single-level memory* means that an OS has no concept of files, only persistent objects are mapped onto the address space of the processes. (https://en.wikipedia.org/wiki/Single-level_store)

#### Linux-License
- Initially published under GNU GPL
- Later licensed under GNU GPL 2 with syscall exception
    - Syscall exception: anything kernel uses via the syscall are not subject to GNU GPL.

#### Linux History
- 1964: Initial Planning and development for Multics by in Cambridge, Massachusetts.
    - A cooperative project led by: MIT, General Electric and Bell Labs.
    - Multics was *developed on* and *designed for* GE 645 comptuer
- 1969: Bell labs withdrew from the project.
- 1969: Ken Thompson and Dennis Ritchie from Bell Labs (AT&T) concieve and implement UNIX.
- 1970: UNIX gets released
- 1973: UNIX is rewritten in C (created by Dennis in 1970) to make it portable
    - UNIX becomes popular:
    - widely adopted, copied and modified by acadimic institutions and businesses.
- 1977: Computer Systems Research Group (CSRG) from UC, Berkely develop Berkeley Software Distribution (BSD).
    - AT&T file lawsuit against UC.
    - Adoption of BSD gets limited
- 1983: Richard Stallman starts GNU project to create UNIX like system
- 1990: Almost enough GNU softwares available for operating system, except kernel.
    - GNU kernel called Hurd doesn't gather enough attention, its growth slowed, making it unfit for good kernel candidate.
- 1987: Parallelly, Andrew S. Tanenbaum developes MINIX, an UNIX like system for academic purposes
    - Although, Source code was public, Modification and Distribution was restricted (initially licensed under proprietary license)
    - 16 bit design made it not the best fit for hardware advances at that time
    - Commercial MINIX for Intel 386 was too expensive for regular users
- 1991: Linus Torvalds, a Finnish Student begins a personal project Linux (kernel)
    - If following things had happened, Linux wouldn't be made:
        - If GNU Hurd or 386 BSD kernels were available at the time.

#### Linux Clarification

Linux by itself represents only a kernel. It is used with with rest of the software stack (like GNU) to make it a complete OS. One example is GNU/Linux which uses Linux kernel with GNU, an extensive collection of Free Softwares like:
- GNU Compiler Collection (GCC)
- GNU C library (glibc)
- GNU Core Utilities (coreutils)
- GNU Debugger (gdb)
- GNU Binary Utilities (binutils)
- GNU Bash shell

GNU can be used with other kernels like Hurd (GNU's own kernel), or FreeBSD. Likewise, linux kernel can be used with other software stacks. Its popular examples are:
- Alpine Linux
- Android
- ChromiumOS/ChromeOS


### Chapter 2. Install Linux and tools [6 Hrs]

### Chapter 3. Access Linux [6 Hrs]

### Chapter 4. Introduction to File System [4 Hrs (Assumed)]

### Chapter 5. File manipulation [4 Hrs]

### Chapter 6. Mastering Vim [6 Hrs]

### Chapter 7. Users/Groups and File Permissions Core Information [4 Hrs]

### Chapter 8. Basic networking [6 Hrs]

### Chapter 9. Manage Basic Storage [10]

### Chapter 10. Advanced Commands (Regular Expressions) [6 Hrs]

### Chapter 11. Basic of Systemd [6 Hrs]

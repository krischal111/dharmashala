
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


#!/bin/bash

# Gonna make 60 total partitions each having 100 Mb

# Assumptions
# This command is executed by root
# /dev/sdb disk exists
# It has enough storage capacity for all 60 parititions


## Script


# Create a new partition table (removing all previous partition)
(echo "o"; # MBR Partitioning
echo "w";  # write
) | sudo fdisk "/dev/sdb"

# Creating the said partitions
for i in $(seq 60)
do
    if [ $i -le 3 ]; then
	# create 3 primary partitions
	(echo "n";  # create a new partition
	echo "p";   # a primary partition
	echo "";    # partition number (will use default value)
	echo "";    # first sector (will use default value)
	echo "+100M"; # last sector (100Mb away, creates partition of 100 Mb)
	echo "w"; # write
	) | sudo fdisk /dev/sdb

    elif [ $i -eq 4 ]; then
	# create extended paritition
	(echo "n";  # create a new partition
	echo "e";   # an extended partition
	echo "";    # partition number (will use default value)
	echo "";    # first sector (will use default value)
	echo "";    # last sector (default value, at the end of the disk)
	echo "w";   # write
	) | sudo fdisk /dev/sdb
    else
	# create more partitions
	(echo "n";  # create a new partition
	echo "";    # first sector (will use default value)
	echo "+100M"; # last sector (100Mb away, creates partition of 100 Mb)
	echo "w"; # write
	) | sudo fdisk /dev/sdb
    fi
done

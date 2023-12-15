# Lab 1: Familiarization with the Microcontroller
## Embedded System
### Expected prior knowledge

* 8051 Microcontroller architecture
* Basic assembly for 8051

- For working under Windows:
    - Using Proteus
- For working in Linux/Mac
    - Using the shell

### Objectives
- Data manipulation in the 8051.
- Arithmetic and Logical Operations.
- Looping and branching techniques.
- Subroutine calls.

### Required softwares:

For windows:
- `Keil` : An IDE for compilation, assembling and debugging 8051 programs.
- `Proteus`: As a simulator.

For linux and MacOS, you will need two softwares:
1. Compiler/Assembler : For compiling c-code or assembly code.
2. Emulator : To emulate the 8051

- Compiler: `sdcc`
    : Install instruction [here](/extra-resources/Softwares/sdcc.md)
- Emulator: `emu8051` : Install instruction [here](/extra-resources/Softwares/emu8051.md)

### Lab questions

[Link to the Solutions](/Subjects/Embedded-System/Labs/Lab-1/)

1. Write code to add the numbers 897F9AH and 34BC48H and save the result in internal RAM starting at 40H. The result should be displayed continuously on the LEDs of the development board starting from least significant byte with an appropriate timing interval between each byte. Use port zero (P0) of the micro-controller to interface
with LEDs.

2. Implement a subroutine that replaces the SWAP instruction using rotate right instructions. Test your program on the contents of the accumulator when it contains the number 6BH.
3. Multiply, by using looping and successive addition technique, the data in RAM location 22H by the data in RAM location 15H and put the result in RAM locations 19H (low byte) and 1AH (high byte). Data in 22H should be FFH and data in 15H should be DEH.

4. Divide, by using looping and successive subtraction technique, the data in RAM location 3EH by the number 12H; put the quotient in R4 and remainder in R5. Data in 3EH should be AFH.

5. Store ten hexadecimal numbers in internal RAM starting from memory location 50H. The list of numbers to be used is: D6H, F2H, E4H, A8H, CEH, B9H, FAH, AEH, BAH, CCH. Implement a subroutine that extracts both the smallest and largest numbers from the stored numbers.

6. Store ten hexadecimal numbers in internal RAM starting from memory location 60H. The list of numbers to be used is: A5H, FDH, 67H, 42H, DFH, 9AH, 84H, 1BH, C7H, 31H. Implement a subroutine that orders the numbers in ascending order using bubble or any other sort algorithm and implement s subroutine that order the numbers in descending order using selection sort algorithm.

7. Store numbers from 00H to 20H in internal RAM starting from memory location 40H. Implement a subroutine that extracts only the prime numbers.

8. Find the factorial of a number stored in R3. The value in R3 could be any number in the range from 00H to 05H. Implement a subroutine that calculates the factorial. The factorial needs to be represented in both hexadecimal and decimal formats.

Solutions:

- [Assembly Codes](/Subjects/Embedded-System/Labs/Lab-1/Assembly-Code/)

#! [Todo]
    
    1. Adding instruction to run the program in Keil and running in Proteus.
    2. Adding instruction to use SDCC to compile, or SDAS8051 to assemble, and using emu8051 to run in the 8051 emulator.
    3. A sample lab report.

    (If you would like to contribute, you're most welcome.)

## Attribution to the contributors:

[Krischal Khanal](https://github.com/krischal111)
# Advanced FPGA and Embedded Design
## Syllabus

Lectures: 3 Teaching Hours per week

Tutorial: 1 Teaching Hours per week

Practical: 3 Teaching Hours per two weeks

## Course Objectives

To provide FPGA Design fundamentals to advance concepts and also understand the
embedded design flow and approaches in FPGA targeted for real world applications.
Understanding latest methodologies and technologies in FPGA, Embedded and VLSI
design.

## Course contents
### Theory
1. FPGA Fundamentals [5 hours]
    1. FPGA overview and its evolution
    2. General FPGA Building Blocks: LUT, FF, DSP, BRAM, I/O, clocks etc.
    3. FPGA Architectures: general architecture, vendor specific architecture summary
    4. Recent developments in FPGA Architecture: heterogeneous architectures
    5. FPGA Architecture targeted for cloud and edge platforms
    6. FPGA Design Flow – summary

2. FPGA logical components, architecture and interfaces [3 hours]
    1. Logical Interconnection and Routing architectures in FPGA
    2. AXI Interface Bus protocol
    3. High speed Interfaces and usage of those interfaces
    4. High speed bus protocols in FPGA : USB, PCIe, Ethernet, MIPI etc.
    5. Embedded SoC/MPSoC architectures detail and interfaces

3. Digital design, simulation and verification with RTL (VHDL/Verilog) [6 hours]
    1. Verilog HDL overview- syntax, semantics, datatypes, primitives, etc.
    2. Behavioral versus structural design modeling
    3. Logical component design with RTL/Verilog and performing simulation: combinational/sequential blocks, FSM, ALU, processor and DSP algorithms
    4. Verification approaches on RTL
    5. RTL design methodologies for FPGA and VLSI Design
    6. Design optimization on RTL for FPGA and VLSI Design

4. Advance RTL design approaches for FPGA [4 hours]
    1. Advance RTL design for latency critical and resource critical designs- overview
    2. Resource, latency, clock and power optimization methodologies
    3. Considerations/approaches for Implementing RTL design in a real-world scenario

5. VLSI Design and Verification [6 hours]
    1. VLSI Design, IC technology, CAD tools on VLSI- overview
    2. VLSI Design flow, design styles and verification methodologies
    3. CMOS Circuit and Logic Design,
    4. Design and analysis of the CMOS inverter
    5. Analog/ Mixed Mode VLSI design concepts

6. Embedded Design methodology on FPGA [4 hours]
    1. Embedded Design overview
    2. RTL and HLS design approaches in Embedded design
    3. SoC/MPSoC FPGAS and design approaches with AMD Xilinx tools

7. Embedded C programming for SoC FPGA [6 hours]
    1. Embedded C programming – overview
    2. Microblaze, ARM processor and RISC based system on chip platforms
    3. Writing applications for embedded SoC and its approaches
    4. Embedded application optimization for latency: multi-threading and multi-processing

8. Embedded Operating Systems and usage on SoC FPGAs [4 hours]
    1. Embedded OS: Baremetal, FreeRTOS, and Linux based OS
    2. FPGA and Embedded design integration with AMD Xilinx SoC/MPSoC FPGA

9. Advanced design, methodologies and applications of FPGA [7 hours]
    1. Recent FPGA developments, architectures and its implications.
    2. application level design, simulation and implementation of DSP, Signal Processing and Telecommunications algorithms in FPGA – Design aspects and methods
    3. RISC processor architecture design flow in Verilog
    4. CMOS component design and analysis
    5. Hardware/software co-design, accelerator design with FPGA and integrating with Embedded flow.
    6. Approaches of algorithms implementation and offloading in FPGA

### Practical
1. Logical design implementation on FPGA with Verilog. Using Arty A7 and Zybo Board.
2. Advanced RTL/Verilog Lab with small scale processor design (8 bit), FFT , DFT, DCT, Convolution and OFDM based design, simulation and implementation in FPGA.
3. RISC-V (ISA) based implementation in RTL/Verilog.
4. Introductory Lab on VLSI design/verification (CAD tool overview, logical gate design, CMOS component design and analysis).
5. Handling GPIO on FPGA platform via Verilog. Using Xilinx Arty A7 and Zybo Board.
6. Embedded FPGA Design Lab- creating IP based design and Embedded C based application with AMD Xilinx VIVADO/Vitis and FPGA.
7. Embedded design for Baremetal , FreeRTOS and Linux based applications.
8. Creating custom accelerator for computer vision and CNN, integrate it with SoC embedded design flow.
9. Using machine learning acceleration approaches in FPGA. Designing with AMD Xilinx platform for Machine Learning Acceleration.

## Books and References
    1. Advanced FPGA Design Architecture, Implementation, and Optimization by Steve Kilts
    2. Advanced Digital Design with the Verilog HDL by Michael Ciletti
    3. FPGA prototyping by Verilog examples by Pong P. Chu
    4. Digital VLSI Design with Verilog by John Williams
    5. CMOS VLSI Design: A Circuits and Systems Perspective by Neil Weste, David Harris
    6. Embedded Systems Design with FPGAs , Editors Peter Athanas, Dionisios Pnevmatikatos, Nicolas Sklavos, Springer Publications

## Evaluation Schemes

The questions will cover all the chapters of the syllabus. The evaluation scheme for the questions will be indicated in the table below:

| S.N.  | Chapter       | Hours          | Marks Distrubution     |
| ----- | ------------- | -------------- | ---------------------- |
| 1     | 1             |  5             |    9                   |
| 2     | 2             |  3             |    5                   |
| 3     | 3             |  6             |   11                   |
| 4     | 4             |  4             |    7                   |
| 5     | 5             |  6             |   11                   |
| 6     | 6             |  4             |    7                   |
| 7     | 7             |  6             |   11                   |
| 8     | 8             |  4             |    7                   |
| 9     | 9             |  7             |   12                   |
| Total |               | 45 hours       |   80                   |

*There may be minor deviation in marks distribution

## Attributions to the Contributors:

[Krischal Khanal](https://github.com/krischal111)
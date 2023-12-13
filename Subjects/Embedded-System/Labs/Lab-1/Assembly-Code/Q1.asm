; Write code to add the numbers 897F9AH and 34BC48H and save the result in internal
; RAM starting at 40H. The result should be displayed continuously on the LEDs of the 
; development board starting from least significant byte with an appropriate timing 
; interval between each byte. Use port zero (P0) of the micro-controller to interface 
; with LEDs

ORG 00H

; Load 897F9AH to register R0, R1, R2
MOV R2, #9AH
MOV R1, #7FH
MOV R0, #89H

; Load 34BC48H to register R3, R4, R5
MOV R5, #48H
MOV R4, #0BCH
MOV R3, #34H

; Add first 8 bits of both data
MOV A, R2
ADD A, R5
MOV 40H, A  ;Store result at memory address 40H of RAM

; Add next 8 bits of both data with carry
MOV A, R1
ADDC A, R4
MOV 41H, A   ;Store result at memory address 41H of RAM

; Add next 8 bits of both data with carry
MOV A, R0
ADDC A, R3
MOV 42H, A    ;Store result at memory address 42H of RAM

; Send data present at memory address 40H,41H,42H to port 0
INFINITELOOP:
	MOV R0, #40H   ;Load the address 40H
	MOV R1, #2H    ;Counter
	LOOP:
		MOV A, @R0
		MOV P0,A
		ACALL DELAY
		INC R0
		DJNZ R1, LOOP
	JMP INFINITELOOP

; Delay Subroutine
DELAY:
	MOV R5, #0FFH
	FIRST_LOOP:
		MOV R6, #0FFH
		SECOND_LOOP:
			MOV R7, #0FFH
			THIRD_LOOP:
				DJNZ R7, THIRD_LOOP
			DJNZ R6, SECOND_LOOP
		DJNZ R5, FIRST_LOOP
    RET

END

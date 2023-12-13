; Store ten hexadecimal numbers in internal RAM starting from memory location 60H.
; The list of numbers to be used is: A5H, FDH, 67H, 42H, DFH, 9AH, 84H, 1BH, 
; C7H, 31H. Implement a subroutine that orders the numbers in ascending order using 
; bubble or any other sort algorithm and implement s subroutine that order the numbers 
; in descending order using selection sort algorithm.

ORG 000H

MOV 60H, #0A5H
MOV 61H, #0FDH
MOV 62H, #067H
MOV 63H, #042H
MOV 64H, #0DFH
MOV 65H, #09AH
MOV 66H, #084H
MOV 67H, #01BH
MOV 68H, #0C7H
MOV 69H, #031H

LCALL SELECTION
LCALL BUBBLE

MOV R0, #60H
MOV R2, #0AH

DISPLAY:
MOV A, @R0
MOV P0, A
INC R0
LCALL DELAY
DJNZ R2, DISPLAY

MOV R0, #60H
MOV R2, #09H
JMP DISPLAY

BUBBLE:
		MOV R2, #09H
		MOV R3, #00H
		OUTER:
		MOV A, #09H
		CLR C
		SUBB A, R3
		MOV R6, A
		MOV R0, #60H
		MOV R1, #61H
			INNER:
				MOV A, @R0
				MOV R4, A
				MOV A, @R1
				CLR C
				SUBB A, R4
				JNC NO_SWAP
				MOV A, @R1
				MOV @R0, A
				MOV A, R4
				MOV @R1, A
				
			NO_SWAP:INC R0 
					INC R1

				DJNZ R6, INNER
				INC R3
		DJNZ R2, OUTER
		RET

SELECTION:
		MOV R0, #60H ;i
		MOV R1, #0H ;j
		MOV R3, #0H ; max
		
		S_OUTER:
			MOV A, R0
			MOV R3, A
			MOV R1, A
			S_INNER:
				INC R1
				CLR C
				MOV A, #6AH
				SUBB A, R1
				JZ INNER_LOOP_EXPIRES
				CLR C
 				MOV A, R0
				MOV R6, A
				MOV A, R3
				MOV R0, A
				MOV A, @R1
				SUBB A, @R0
				MOV A, R6
				MOV R0, A
				JC NOT_MAX
				MOV A, R1
				MOV R3, A
				NOT_MAX:
				SJMP S_INNER
				INNER_LOOP_EXPIRES:
				;swap @R3 and @R0
				MOV A, R1
				MOV R6, A
				MOV A, R3
				MOV R1, A
				MOV A, @R1
				MOV R4, A
				MOV A, @R0
				MOV @R1, A
				MOV A, R4
				MOV @R0, A
				MOV A, R1
				MOV R3, A
				MOV A, R6
				MOV R1, A
				INC R0
				CLR C
				MOV A, #69H
				DEC A ;lol just mov A, #68H
				CLR C
				SUBB A, R0
				JNC S_OUTER
				RET
	
DELAY:
MOV R3, #0FFH
DOUTER:
MOV R4, #0FFH 
DINNER:
DJNZ R4, DINNER
DJNZ R3, DOUTER
RET

END
; Store ten hexadecimal numbers in internal RAM starting from memory location 50H.
; The list of numbers to be used is: D6H, F2H, E4H, A8H, CEH, B9H, FAH, AEH, 
; BAH, CCH. Implement a subroutine that extracts both the smallest and largest 
; numbers from the stored numbers.

ORG 00H

; Store given data in internal RAM
MOV 50H, #0D6H
MOV 51H, #0F2H
MOV 52H, #0E4H
MOV 53H, #0A8H
MOV 54H, #0CEH
MOV 55H, #0B9H
MOV 56H, #0FAH
MOV 57H, #0AEH
MOV 58H, #0BAH
MOV 59H, #0CCH

; Subroutine to extract largest and smallest
; Store largest number in R5 and smallest in R6
CHECK:
	MOV R0, #0AH  ; Counter for 10 numbers
	MOV R1, #50H  ; Starting address
	MOV R5, 50H  ; Initial value as largest number
	MOV R6, 50H  ; Initial value as smallest number
	LOOP:
		
		; Check for greater
		MOV A, R5
		SUBB A, @R1
		JNC NOT_GREATER
		MOV A, @R1
		MOV R5, A
		NOT_GREATER:
		
		; Check for smaller
		MOV A, R6
		SUBB A, @R1
		JC NOT_SMALLER
		MOV A, @R1
		MOV R6, A
		NOT_SMALLER:
		
		INC R1
		DJNZ R0, LOOP
		
	RET

ACALL CHECK
END
		
		
		

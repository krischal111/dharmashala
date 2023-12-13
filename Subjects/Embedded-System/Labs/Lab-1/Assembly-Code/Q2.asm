; Implement a subroutine that replaces the SWAP instruction using rotate right 
; instructions. Test your program on the contents of the accumulator when it contains 
; the number 6BH

ORG 00H
;Check subroutine using data
MOV R0, #6BH

;Subroutine for SWAP
SWAP_SUB:
	MOV A,R0
	RRC A
	RRC A
	RRC A
	RRC A
	MOV R0,A
	RET
	
ACALL SWAP_SUB
MOV A, #6BH

END
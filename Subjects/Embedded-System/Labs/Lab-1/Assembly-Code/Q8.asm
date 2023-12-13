; Find the factorial of a number stored in R3. The value in R3 could be any number in 
; the range from 00H to 05H. Implement a subroutine that calculates the factorial. The 
; factorial needs to be represented in both hexadecimal and decimal formats

ORG 00H

;Store number in R3
MOV R3, #5H

MOV A, R3
MOV R0, A  ;Counter

;Subroutine to calculate factorial
FACT:
	DEC R0
	MOV B,R0
	MUL AB
	CJNE R0, #1H, FACT
;Result in A
;Convert to decimal format
MOV R0, #65H   ;store decimal number from address 65h
LOOP:
	MOV B,#0AH
	DIV AB
	MOV R1, A
	MOV A, B
	MOV @R0, A
	MOV A, R1
	INC R0
	JNZ LOOP
END
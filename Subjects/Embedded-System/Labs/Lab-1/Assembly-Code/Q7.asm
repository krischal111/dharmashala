; Store numbers from 00H to 20H in internal RAM starting from memory location 40H.
; Implement a subroutine that extracts only the prime numbers

ORG 00H

; Store numbers from 00H to 20H in internal RAM 
; starting from memory location 40H
MOV A, #00H  ;starting number
MOV R1, #021H
MOV R0, #40H  ;starting memory location for datas

; store numbers from 00h to 20h
INSERT:
	MOV @R0, A
	INC R0
	INC A
	DJNZ R1, INSERT

; Store prime numbers from memory location 65H
MOV R0, #040H   ;starting memory location for datas
MOV R1, #065H   ;starting memory location for prime number
MOV R3, #021H   ;counter

MOV R2, #00H    ;no. of prime number 

CHECK_PRIME:

   ; for 1 and 0
	CLR C
	MOV A, @R0
	SUBB A, #02H
	JC NOT_PRIME
	
	MOV A, @R0
	MOV B, #02H
	DIV AB
	MOV R4, A
	MOV R6, #01H
	
	MAIN_LOOP: 
	INC R6	
	CLR C
	MOV A, R4
	SUBB A, R6
	JC IS_PRIME
	
	MOV A, @R0
	MOV B, R6
	DIV AB
	MOV A, B
	CLR C
	SUBB A, #0H
	JZ NOT_PRIME

	SJMP MAIN_LOOP
	
	IS_PRIME: 
		INC R2 
		MOV A, @R0
		MOV @R1, A
		INC R1
		
	NOT_PRIME:
		INC R0
		
DJNZ R3, CHECK_PRIME

END
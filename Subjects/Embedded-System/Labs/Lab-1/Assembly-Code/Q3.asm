; Multiply, by using looping and successive addition technique, the data in RAM 
; location 22H by the data in RAM location 15H and put the result in RAM locations 
; 19H (low byte) and 1AH (high byte). Data in 22H should be FFH and data in 15H 
; should be DEH.

ORG 00H

;Set Data
MOV 22H, #0FFH
MOV 15H, #0DEH

;Result
MOV 19H, #0H
MOV 1AH, #0H

MOV A, #0H
MOV R1, 15H  ;counter/multliplier

; Multiplication of two 8-bit result in max 16-bit
; For upper byte
MOV R0, #00H

;Multiplication by successive addition
LOOP:
	
	MOV A, 19H  ;Memory address 19H for lower byte
	ADD A, 22H
	MOV 19H, A
	
	MOV A, 1AH  ;Memory address 1AH for high byte
	ADDC A,#0H  ;add R0 and carry bit
	MOV 1AH, A
	
	DJNZ R1, LOOP

END
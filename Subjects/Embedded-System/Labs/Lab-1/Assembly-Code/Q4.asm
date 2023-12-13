; Divide, by using looping and successive subtraction technique, the data in RAM 
; location 3EH by the number 12H; put the quotient in R4 and remainder in R5. Data in 
; 3EH should be AFH.

;Dividend-Memory(3EH), Divisor-Number(12H)
;Result : Quotient(R4), Remainder(R5)

ORG 00H

; Store dividend data
MOV 3EH, #0AFH

MOV A, 3EH
MOV R0, #12H
MOV R4, #0H  ;Quotient
MOV R5, #0H  ;Remainder

SUBB A,R0
LOOP:
	INC R4
	SUBB A, R0
	JNC LOOP
	
ADD A,R0  ;To get remainder
MOV R5,A

END
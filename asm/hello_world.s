    .org $0
DIS_OUT .equ $200
    .org $400

main:
    LDX #$0
print_char:
    LDA message,x
    BEQ do_nothing_loop
    STA DIS_OUT  
    INX
    JMP print_char
do_nothing_loop:
    NOP
    JMP do_nothing_loop; later use nothing label


message: .byte "Hello, World!", $00
    .org $FFFC
    .word main

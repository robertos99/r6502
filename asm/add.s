    .org $0
DIS_OUT .equ $200
    .org $400

main:
    LDA #$2 ; number 2
    ADC #$46 ; add 46 to get ascii H
    ; STA $F0
    STA DIS_OUT
    LDA #"e"
    STA DIS_OUT
    LDA #"l"
    STA DIS_OUT
    LDA #"l"
    STA DIS_OUT
    LDA #"o"
    STA DIS_OUT
    LDA #" "
    STA DIS_OUT
    LDA #"W"
    STA DIS_OUT
    LDA #"o"
    STA DIS_OUT
    LDA #"r"
    STA DIS_OUT
    LDA #"l"
    STA DIS_OUT
    LDA #"d"
    STA DIS_OUT
    LDA #"!"
    STA DIS_OUT
do_nothing_loop:
    NOP
    JMP do_nothing_loop; later use nothing label

    .org $FFFC
    .word main

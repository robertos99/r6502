    LDA #$2 ; number 2
    ADC #$46 ; add 46 to get ascii H
    ; STA $F0
    STA $200
    LDA #"e"
    STA $200
    LDA #"l"
    STA $200
    LDA #"l"
    STA $200
    LDA #"o"
    STA $200
    LDA #" "
    STA $200
    LDA #"W"
    STA $200
    LDA #"o"
    STA $200
    LDA #"r"
    STA $200
    LDA #"l"
    STA $200
    LDA #"d"
    STA $200
    LDA #"!"
    STA $200
n:
    NOP
    JMP n; later use nothing label
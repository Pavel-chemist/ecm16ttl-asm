.const
    input_address = 0x0100

.text
    main:
        LDd r1 input_address
        ADDi r1 number          #adds number to register r1

.data

    #some comments

    number: 
        .word 0x001F

    other_num:
        .dword 0x12345678
 
    text: .string   "12345 q 12345 \"12345\""

    double_byte:  .word  0x31



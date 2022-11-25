.const
    input_address = 0x0010

.text
    LDd r1 input_address
    ADDi r1 number          #adds number to register r1

.data
    number: .word 0x1F



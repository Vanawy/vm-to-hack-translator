// Push Constant 0
@0
D=A
@SP
A=M
M=D
@SP
M=M+1
// Pop Local 0
@LCL
D=M
@R13
M=D
@SP
AM=M-1
D=M
@R13
A=M
M=D
// Label loop_start
(BasicLoop.vm$loop_start)
// Push Argument 0
@ARG
D=M
A=D
D=M
@SP
A=M
M=D
@SP
M=M+1
// Push Local 0
@LCL
D=M
A=D
D=M
@SP
A=M
M=D
@SP
M=M+1
// Add
@SP
AM=M-1
D=M
@SP
AM=M-1
D=D+M
@SP
A=M
M=D
@SP
M=M+1
// Pop Local 0
@LCL
D=M
@R13
M=D
@SP
AM=M-1
D=M
@R13
A=M
M=D
// Push Argument 0
@ARG
D=M
A=D
D=M
@SP
A=M
M=D
@SP
M=M+1
// Push Constant 1
@1
D=A
@SP
A=M
M=D
@SP
M=M+1
// Subtract
@SP
AM=M-1
D=M
@SP
AM=M-1
D=M-D
@SP
A=M
M=D
@SP
M=M+1
// Pop Argument 0
@ARG
D=M
@R13
M=D
@SP
AM=M-1
D=M
@R13
A=M
M=D
// Push Argument 0
@ARG
D=M
A=D
D=M
@SP
A=M
M=D
@SP
M=M+1
// IfGoto loop_start
@SP
AM=M-1
D=M
@BasicLoop.vm$loop_start
D;JGT
// Push Local 0
@LCL
D=M
A=D
D=M
@SP
A=M
M=D
@SP
M=M+1

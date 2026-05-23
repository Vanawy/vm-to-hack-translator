// Push Constant 3030
@3030
D=A
@SP
A=M
M=D
@SP
M=M+1
// Pop Pointer 0
@SP
AM=M-1
D=M
@THIS
M=D
// Push Constant 3040
@3040
D=A
@SP
A=M
M=D
@SP
M=M+1
// Pop Pointer 1
@SP
AM=M-1
D=M
@THAT
M=D
// Push Constant 32
@32
D=A
@SP
A=M
M=D
@SP
M=M+1
// Pop This 2
@THIS
D=M
@2
D=D+A
@R13
M=D
@SP
AM=M-1
D=M
@R13
A=M
M=D
// Push Constant 46
@46
D=A
@SP
A=M
M=D
@SP
M=M+1
// Pop That 6
@THAT
D=M
@6
D=D+A
@R13
M=D
@SP
AM=M-1
D=M
@R13
A=M
M=D
// Push Pointer 0
@THIS
D=M
@SP
A=M
M=D
@SP
M=M+1
// Push Pointer 1
@THAT
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
// Push This 2
@THIS
D=M
@2
A=D+A
D=M
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
// Push That 6
@THAT
D=M
@6
A=D+A
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
// Push Constant 111
@111
D=A
@SP
A=M
M=D
@SP
M=M+1
// Push Constant 333
@333
D=A
@SP
A=M
M=D
@SP
M=M+1
// Push Constant 888
@888
D=A
@SP
A=M
M=D
@SP
M=M+1
// Pop Static 8
@SP
AM=M-1
D=M
@StaticTest.vm.8
M=D
// Pop Static 3
@SP
AM=M-1
D=M
@StaticTest.vm.3
M=D
// Pop Static 1
@SP
AM=M-1
D=M
@StaticTest.vm.1
M=D
// Push Static 3
@StaticTest.vm.3
D=M
@SP
A=M
M=D
@SP
M=M+1
// Push Static 1
@StaticTest.vm.1
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
// Push Static 8
@StaticTest.vm.8
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
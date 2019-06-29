// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

// Put your code here.

@idx // set index to 0
M=0
@diff
M=0

(START)

@KBD
D=M
@WHITE
D;JEQ // jump to white if KBD word is 0 (no key pressed)

(BLACK)
@diff
M=1
@idx
D=M
@SCREEN
A=A+D
M=0
M=!M
@diff
D=M
@idx
M=M+D // move the counter
@LOOP
0;JMP

(WHITE)
@diff
M=-1
@idx
D=M
@SCREEN
A=A+D
M=0

@idx
D=M
@LOOP
D;JEQ
@diff
D=M
@idx
M=M+D // move the counter

(LOOP)
@START
0;JMP

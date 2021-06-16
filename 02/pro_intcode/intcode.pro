:- use_module(library(clpfd)).

data([1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,10,19,1,6,19,23,2,23,6,27,2,6,27,31,2,13,31,35,1,10,35,39,2,39,13,43,1,43,13,47,1,6,47,51,1,10,51,55,2,55,6,59,1,5,59,63,2,9,63,67,1,6,67,71,2,9,71,75,1,6,75,79,2,79,13,83,1,83,10,87,1,13,87,91,1,91,10,95,2,9,95,99,1,5,99,103,2,10,103,107,1,107,2,111,1,111,5,0,99,2,14,0,0]).

result(19690720).

opcode(add, 1).
opcode(mul, 2).
opcode(quit, 99).

interpret(IP, M, O) :-
	nth0(IP, M, OP),
	interpret_(OP, IP, M, O).

interpret_(OP, _, M, M) :- opcode(quit, OP).
interpret_(OP, IP, M, O) :-
	IP1 #= IP + 1,
	IP2 #= IP + 2,
	nth0(IP1, M, PA),
	nth0(IP2, M, PB),
	nth0(PA, M, A),
	nth0(PB, M, B),
	instruction(OP, A, B, R),
	IP3 #= IP + 3,
	nth0(IP3, M, RESULT_ADDRESS),
	replace(RESULT_ADDRESS, R, M, M1),
	IP4 #= IP + 4,
	interpret(IP4, M1, O).

instruction(OP, _, _, _) :- opcode(quit, OP).
instruction(OP, A, B, C) :- opcode(add, OP), C #= A + B.
instruction(OP, A, B, C) :- opcode(mul, OP), C #= A * B.

replace(N, Elem, As, Bs) :-
	nth0(N, As, _, T1),
	nth0(N, Bs, Elem, T1).

problem_b(X) :-
	Noun #>= 0,
	Noun #=< 99,
	Verb #>= 0,
	Verb #=< 99,
	data(M),
	replace(1, Noun,  M, M1),
	replace(2, Verb, M1, M2),
	interpret(0, M2, M3),
	result(R),
	nth0(0, M3, R),
	X #= 100 * Noun + Verb.

problem_a(X) :-
	data(M),
	replace(1, 12,  M, M1),
	replace(2,  2, M1, M2),
	interpret(0, M2, [X | _]).

answer_a :-
	problem_a(X),
	writeln(X).

answer_b :-
	problem_b(X),
	writeln(X).

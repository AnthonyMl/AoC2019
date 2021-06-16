:- use_module(library(clpfd)).

data([3,225,1,225,6,6,1100,1,238,225,104,0,1102,78,40,225,1102,52,43,224,1001,224,-2236,224,4,224,102,8,223,223,101,4,224,224,1,224,223,223,1,191,61,224,1001,224,-131,224,4,224,102,8,223,223,101,4,224,224,1,223,224,223,1101,86,74,225,1102,14,76,225,1101,73,83,224,101,-156,224,224,4,224,102,8,223,223,101,6,224,224,1,224,223,223,1102,43,82,225,2,196,13,224,101,-6162,224,224,4,224,102,8,223,223,101,5,224,224,1,223,224,223,1001,161,51,224,101,-70,224,224,4,224,102,8,223,223,1001,224,1,224,1,224,223,223,102,52,187,224,1001,224,-832,224,4,224,102,8,223,223,101,1,224,224,1,224,223,223,1102,19,79,225,101,65,92,224,1001,224,-147,224,4,224,1002,223,8,223,101,4,224,224,1,223,224,223,1102,16,90,225,1102,45,44,225,1102,92,79,225,1002,65,34,224,101,-476,224,224,4,224,102,8,223,223,1001,224,5,224,1,224,223,223,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,107,226,226,224,1002,223,2,223,1005,224,329,1001,223,1,223,1007,226,226,224,102,2,223,223,1005,224,344,101,1,223,223,1008,226,226,224,102,2,223,223,1005,224,359,1001,223,1,223,8,226,677,224,102,2,223,223,1006,224,374,101,1,223,223,1107,226,677,224,1002,223,2,223,1006,224,389,101,1,223,223,1108,226,677,224,102,2,223,223,1005,224,404,101,1,223,223,107,677,677,224,102,2,223,223,1006,224,419,1001,223,1,223,7,677,226,224,102,2,223,223,1005,224,434,101,1,223,223,1007,677,677,224,102,2,223,223,1005,224,449,1001,223,1,223,108,226,677,224,102,2,223,223,1005,224,464,1001,223,1,223,108,226,226,224,102,2,223,223,1006,224,479,101,1,223,223,107,226,677,224,102,2,223,223,1006,224,494,1001,223,1,223,7,226,226,224,1002,223,2,223,1006,224,509,101,1,223,223,1108,677,226,224,102,2,223,223,1005,224,524,101,1,223,223,1107,677,226,224,102,2,223,223,1005,224,539,101,1,223,223,1008,677,226,224,102,2,223,223,1005,224,554,101,1,223,223,1008,677,677,224,1002,223,2,223,1006,224,569,101,1,223,223,1107,677,677,224,102,2,223,223,1006,224,584,1001,223,1,223,1108,226,226,224,1002,223,2,223,1006,224,599,101,1,223,223,7,226,677,224,102,2,223,223,1006,224,614,101,1,223,223,108,677,677,224,1002,223,2,223,1006,224,629,101,1,223,223,1007,677,226,224,102,2,223,223,1006,224,644,101,1,223,223,8,677,677,224,1002,223,2,223,1006,224,659,101,1,223,223,8,677,226,224,102,2,223,223,1005,224,674,101,1,223,223,4,223,99,226]).

inst(  add, opcode( 1), num_args(3)).
inst(  mul, opcode( 2), num_args(3)).
inst( read, opcode( 3), num_args(1)).
inst(write, opcode( 4), num_args(1)).
inst(   jt, opcode( 5), num_args(2)).
inst(   jf, opcode( 6), num_args(2)).
inst(   lt, opcode( 7), num_args(3)).
inst(   eq, opcode( 8), num_args(3)).
inst( quit, opcode(99), num_args(0)).

mode(position, 0).
mode(immediate, 1).

interpret(IP, (Is, OsI, M), (OsF, MF)) :-
	nth0(IP, M, X),
	num_modes_opcode(X, Modes, OPC),
	inst(OP, opcode(OPC), num_args(NA)),
	extend(NA, Modes, Modes1),
	fix_modes(OP, Modes1, Modes2),
	args(M, Modes2, IP, Args),
	quit_trap(OP, Args, IP, (Is, OsI, M), (OsF, MF)).

% HACK for when the last argument is a write address.
% In this case we want it to always be in immediate mode even if reported in position.
% Exceptions for instructions that do not write to memory.
% Their last parameter may be position.
%
fix_modes(OP, [X,XA|Xs], [X,YA|Ys]) :- fix_modes(OP, [XA|Xs], [YA|Ys]).
fix_modes(OP, [position], [immediate]) :- OP \= write, OP \= jf, OP \= jt.
fix_modes(write, [position], [position]).
fix_modes(jf, [position], [position]).
fix_modes(jt, [position], [position]).
fix_modes( _, [immediate], [immediate]).
fix_modes( _, [], []).

quit_trap(quit, [], _, (_, Os, M), (Os, M)).
quit_trap(OP, As, IP, (Is, Os, M), (OsF, MF)) :-
	OP \= quit,
	instr_(OP, As, (Is, Os, M, IP), (Is1, Os1, M1, IP1)),
	interpret(IP1, (Is1, Os1, M1), (OsF, MF)).

args(M, [immediate | Modes], IP, [A | As]) :-
	IP1 #= IP + 1,
	nth0(IP1, M, A),
	args(M, Modes, IP1, As).
args(M, [position | Modes], IP, [A | As]) :-
	IP1 #= IP + 1,
	nth0(IP1, M, P),
	nth0(P, M, A),
	args(M, Modes, IP1, As).
args(_, [], _, []).

extend(N, [X|Xs], [X|Ys]) :-
	N #> 0,
	N1 #= N - 1,
	extend(N1, Xs, Ys).
extend(N, [], [position|Ys]) :-
	N #> 0,
	N1 #= N - 1,
	extend(N1, [], Ys).
extend(0, [], []).

% TODO: try arg/3 for array access

instr_(add, [A, B, RP], (Is, Os, M, IP), (Is, Os, MF, IPF)) :-
	R #= A + B,
	inst(add, _, num_args(NA)), IPF #= IP + NA + 1,
	replace(RP, R, M, MF).
instr_(mul, [A, B, RP], (Is, Os, M, IP), (Is, Os, MF, IPF)) :-
	R #= A * B,
	inst(mul, _, num_args(NA)), IPF #= IP + NA + 1,
	replace(RP, R, M, MF).
instr_(read, [RP], ([I|Is], Os, M, IP), (Is, Os, MF, IPF)) :-
	inst(read, _, num_args(NA)), IPF #= IP + NA + 1,
	replace(RP, I, M, MF).
instr_(write, [O], (Is, Os, M, IP), (Is, [O|Os], M, IPF)) :-
	inst(write, _, num_args(NA)), IPF #= IP + NA + 1,
	write(O),nl.
instr_(jt, [X, IP], (Is, Os, M, _), (Is, Os, M, IP)) :-
	X \= 0.
instr_(jt, [0, _], (Is, Os, M, IP), (Is, Os, M, IPF)) :-
	inst(jt, _, num_args(NA)), IPF #= IP + NA + 1.
instr_(jf, [X, _], (Is, Os, M, IP), (Is, Os, M, IPF)) :-
	X \= 0,
	inst(jf, _, num_args(NA)), IPF #= IP + NA + 1.
instr_(jf, [0, IP], (Is, Os, M, _), (Is, Os, M, IP)).
instr_(lt, [A, B, RP], (Is, Os, M, IP), (Is, Os, MF, IPF)) :-
	A #<  B,
	inst(lt, _, num_args(NA)), IPF #= IP + NA + 1,
	replace(RP, 1, M, MF).
instr_(lt, [A, B, RP], (Is, Os, M, IP), (Is, Os, MF, IPF)) :-
	A #>= B,
	inst(lt, _, num_args(NA)), IPF #= IP + NA + 1,
	replace(RP, 0, M, MF).
instr_(eq, [A, B, RP], (Is, Os, M, IP), (Is, Os, MF, IPF)) :-
	A #= B,
	inst(eq, _, num_args(NA)), IPF #= IP + NA + 1,
	replace(RP, 1, M, MF).
instr_(eq, [A, B, RP], (Is, Os, M, IP), (Is, Os, MF, IPF)) :-
	A #\= B,
	inst(eq, _, num_args(NA)), IPF #= IP + NA + 1,
	replace(RP, 0, M, MF).

replace(N, Elem, As, Bs) :-
	nth0(N, As, _, T1),
	nth0(N, Bs, Elem, T1).

modes(0, []).
modes(Number, [Mode | Modes]) :-
	Number #> 0,
	Rest #= Number  // 10,
	ModeId #= Number mod 10,
	mode(Mode, ModeId),
	modes(Rest, Modes).

num_modes_opcode(N, Modes, OP) :-
	M #= N // 100,
	modes(M, Modes),
	OP #= N - M * 100.

problem_a() :-
	data(M),
	interpret(0, ([1], [], M), _).

problem_b() :-
	data(M),
	interpret(0, ([5], [], M), _).



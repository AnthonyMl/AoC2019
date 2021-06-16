int: FIRST = 1;
int: LAST = 6;
set of int: LENGTH = FIRST..LAST;
set of int: DIGIT = 0..9;

array[LENGTH] of var DIGIT: password;

var int: value = sum(i in LENGTH)(password[i] * pow(10, (6-i)));

constraint value >= 356261;
constraint value <= 846303;

constraint forall(i in FIRST..LAST-1)(password[i] <= password[i+1]);

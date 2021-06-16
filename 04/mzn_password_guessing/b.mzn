include "common.mzn";

set of int: DIFFERENCE_LENGTH = FIRST..LAST-1;
set of int: DIFFERENCE_RANGE = -9..9;
array[DIFFERENCE_LENGTH] of var DIFFERENCE_RANGE: differences;

constraint forall(i in DIFFERENCE_LENGTH)(differences[i] = password[i] - password[i+1]);

constraint
  (differences[FIRST]  == 0 /\ differences[FIRST+1] != 0) \/
  (differences[LAST-1] == 0 /\ differences[LAST-2] != 0) \/
  exists(i in FIRST+1..LAST-2)(
    differences[i-1] != 0 /\
    differences[i]   == 0 /\
    differences[i+1] != 0);

solve satisfy;

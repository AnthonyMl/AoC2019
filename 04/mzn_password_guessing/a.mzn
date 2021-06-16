include "common.mzn";

constraint exists(i in FIRST..LAST-1)(password[i] == password[i+1]);

solve satisfy;

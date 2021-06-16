Run with

minizinc -a -s -o <output_file> <input> | select-string 'nSolutions'

where input is a.mzn or b.mzn

e.g.
    minizinc -a -s -o NUL a.mzn | select-string 'nSolutions'

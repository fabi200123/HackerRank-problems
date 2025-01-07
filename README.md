## HackerRank problems

The following problems are solved from the HackerRank using Rust language.

**NOTE**: Those problems are solved as a task for the ATAD course from the UPT Master IT, 2nd Year.

### Easy Problems

1. [Solve Me First](https://www.hackerrank.com/challenges/solve-me-first/problem?isFullScreen=true)

**Solution**: [easy/solve_me_first.rs](https://github.com/fabi200123/HackerRank-problems/blob/master/easy/solve_me_first.rs)

**NOTE**: The problem requires to print the sum of 2 numbers.

2. [Simple Array Sum](https://www.hackerrank.com/challenges/simple-array-sum/problem?isFullScreen=true)

**Solution**: [easy/simple_array_sum.rs](https://github.com/fabi200123/HackerRank-problems/blob/master/easy/simple_array_sum.rs)

**NOTE**: The problem requires me to complete the function, by iterating over the elements of an array and returning the sum of those.

3. [A Very Big Sum](https://www.hackerrank.com/challenges/a-very-big-sum/problem?isFullScreen=true)

**Solution**: [easy/a_very_big_sum.rs](https://github.com/fabi200123/HackerRank-problems/blob/master/easy/a_very_big_sum.rs)

**NOTE**: The problem requires me to complete the function for calculating the sum of an array composed of Big integers. Solved by iterating over the array and adding them together.

4. [Diagonal Difference](https://www.hackerrank.com/challenges/diagonal-difference/problem?isFullScreen=true)

**Solution**: [easy/diagonal_difference.rs](https://github.com/fabi200123/HackerRank-problems/blob/master/easy/diagonal_difference.rs)

**NOTE**: The problem requires me to complete the function for calculating the absolute difference of the 2 sums from each diagonal of a n x n matrix. For this, I have calculated the sum for each diagonal and used `abs()` on the resulted difference of the 2 sums.

5. [CamelCase](https://www.hackerrank.com/challenges/camelcase/problem?isFullScreen=true)

**Solution**: [easy/camelCase.rs](https://github.com/fabi200123/HackerRank-problems/blob/master/easy/camelCase.rs)

**NOTE**: The problem requires me to complete the function for calculating the number of words from the string s (CamelCase). For this I start the sum from 1 and add 1 for each upper case letter.

6. [Grading Students](https://www.hackerrank.com/challenges/grading/problem?isFullScreen=true)

**Solution**: [easy/grading_students.rs](https://github.com/fabi200123/HackerRank-problems/blob/master/easy/grading_students.rs)

**NOTE**: The problem requires me to complete the function for rounding the grades for the students and returing a vector with the updated grades. Firstly, I check if the grade of the student is lower than 38 (for this case, the grade doesn't rounds - student is considered failed), after that I calculate the result of the division to 5 of the grade, which I round and based on the diference of the grade and the result of the (rounded division * 5), I compare if the grade should be / should not be rounded up. Thus, storing the results in a new vector and returning it.

7. [Staircase](https://www.hackerrank.com/challenges/staircase/problem?isFullScreen=true)

**Solution**: [easy/staircase.rs](https://github.com/fabi200123/HackerRank-problems/blob/master/easy/staircase.rs)

**NOTE**: The problem requires me generate a staircase in the format below. In order to achieve this, I have created an iteration from 1 to `n+1`, in which I have printed `n-i` spaces and `i` `#`. Thus, creating the staircase.

```
   #
  ##
 ###
####
```

### Medium Problems

1. [Extra Long Factorials](https://www.hackerrank.com/challenges/extra-long-factorials/problem?isFullScreen=true)

**Solution**: [medium/extra_long_factorials.rs](https://github.com/fabi200123/HackerRank-problems/blob/master/medium/extra_long_factorials.rs)

**NOTE**: The problem requires me to do the factorial of the integer n (`1 <= n <= 100`). For this, I use the BigInt from num library to store the result and I do a `for 1..=n` and do the product of all elements.

2. [Encryption](https://www.hackerrank.com/challenges/encryption/problem?isFullScreen=true)

**Solution**: [medium/encryption.rs](https://github.com/fabi200123/HackerRank-problems/blob/master/medium/encryption.rs)

**NOTE**: The problem requires me to transform a string into an encrypted script. For this, I first need to remove the spaces from the message. Then I calculate the sqrt of the length of the string, based on floor/ceil I calculate the rows/columns. Based on the rows and columns and iterating step by step over the characters, I recreate the words for the encrypted message, which I return with spaces between these words.

### Hard Problems
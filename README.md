## HackerRank problems

The following problems are solved from the HackerRank using Rust language.

**NOTE**: Those problems are solved as a task for the ATAD course from the UPT Master IT, 2nd Year.

### Easy Problems

1. [Solve Me First](https://www.hackerrank.com/challenges/solve-me-first/problem?isFullScreen=true)

**Solution**: [easy/solvemefirst.rs](https://github.com/fabi200123/HackerRank-problems/blob/master/easy/solvemefirst.rs)

**NOTE**: The problem requires to print the sum of 2 numbers.

2. [Simple Array Sum](https://www.hackerrank.com/challenges/simple-array-sum/problem?isFullScreen=true)

**Solution**: [easy/simplearraysum.rs](https://github.com/fabi200123/HackerRank-problems/blob/master/easy/simplearraysum.rs)

**NOTE**: The problem requires me to complete the function, by iterating over the elements of an array and returning the sum of those.

3. [A Very Big Sum](https://www.hackerrank.com/challenges/a-very-big-sum/problem?isFullScreen=true)

**Solution**: [easy/averybigsum.rs](https://github.com/fabi200123/HackerRank-problems/blob/master/easy/averybigsum.rs)

**NOTE**: The problem requires me to complete the function for calculating the sum of an array composed of Big integers. Solved by iterating over the array and adding them together.

4. [Diagonal Difference](https://www.hackerrank.com/challenges/diagonal-difference/problem?isFullScreen=true)

**Solution**: [easy/diagonaldifference.rs](https://github.com/fabi200123/HackerRank-problems/blob/master/easy/diagonaldifference.rs)

**NOTE**: The problem requires me to complete the function for calculating the absolute difference of the 2 sums from each diagonal of a n x n matrix. For this, I have calculated the sum for each diagonal and used `abs()` on the resulted difference of the 2 sums.

5. [CamelCase](https://www.hackerrank.com/challenges/camelcase/problem?isFullScreen=true)

**Solution**: [easy/camelcase.rs](https://github.com/fabi200123/HackerRank-problems/blob/master/easy/camelcase.rs)

**NOTE**: The problem requires me to complete the function for calculating the number of words from the string s (CamelCase). For this I start the sum from 1 and add 1 for each upper case letter.

### Medium Problems

1. [Extra Long Factorials](https://www.hackerrank.com/challenges/extra-long-factorials/problem?isFullScreen=true)

**Solution**: [easy/extra_long_factorials.rs](https://github.com/fabi200123/HackerRank-problems/blob/master/medium/extra_long_factorials.rs)

**NOTE**: The problem requires me to do the factorial of the integer n (`1 <= n <= 100`). For this, I use the BigInt from num library to store the result and I do a `for 1..=n` and do the product of all elements.


### Hard Problems
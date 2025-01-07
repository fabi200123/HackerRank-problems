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

8. [Birthday Cake Candles](https://www.hackerrank.com/challenges/birthday-cake-candles/problem?isFullScreen=true)

**Solution**: [easy/birthday_cake_candles.rs](https://github.com/fabi200123/HackerRank-problems/blob/master/easy/birthday_cake_candles.rs)

**NOTE**: The problem requires me calculate the number of occurences of the biggest element in a vector. So firstly, I create an aux where I store the first element of the vector, then if the element stored in the aux occurs in the vector, I increase the count. If the aux is smaller than an element of the vector, aux takes the value of that element and reset count to 1. After that, continue iteration.

9. [Compare the Triplets](https://www.hackerrank.com/challenges/compare-the-triplets/problem?isFullScreen=true)

**Solution**: [easy/compare_the_triplets.rs](https://github.com/fabi200123/HackerRank-problems/blob/master/easy/compare_the_triplets.rs)

**NOTE**: The problem requires me to compare in order the elements of 2 vectors. So I create a for loop and I compare element by element the 2 triplets. If one element is bigger, the correspoding array receives a point (stored in variable `score_<array-name>`).

10. [Bill Division](https://www.hackerrank.com/challenges/bon-appetit/problem?isFullScreen=true)

**Solution**: [easy/bill_division.rs](https://github.com/fabi200123/HackerRank-problems/blob/master/easy/bill_division.rs)

**NOTE**: The problem requires me to check if the sum of all elements of a vector, minus a specific element, divided by 2 is equal to another number. I have used a `for` to iterate over the array and using an `if` to skip the element that is not needed to be counted and storing this sum. With this sum, I have calculated the value needed to be payed and compared it to the number given, printing the corresponding output.

11. [The Hurdle Race](https://www.hackerrank.com/challenges/the-hurdle-race/problem?isFullScreen=true)

**Solution**: [easy/the_hurdle_race.rs](https://github.com/fabi200123/HackerRank-problems/blob/master/easy/the_hurdle_race.rs)

**NOTE**: The problem requires me to check the number of potions needed by the knight. For this, I have a `for` that iterates over the hurdles heights, and with an `while`, I increase the number of potions and the jump height of the knight, if the hurdle is too big for him to jump. In the end, I return the number of potions used.

12. [Utopian Tree](https://www.hackerrank.com/challenges/utopian-tree/problem?isFullScreen=true)

**Solution**: [easy/utopian_tree.rs](https://github.com/fabi200123/HackerRank-problems/blob/master/easy/utopian_tree.rs)

**NOTE**: The problem requires me to output the height of the tree. In order to do this, I have to add `+1` for each odd cycle, and to double the height for each even cycle. I have created a for to iterate over the cycles and based on the rules mentioned before, I have calculated the height.

13. [Divisible Sum Pairs](https://www.hackerrank.com/challenges/divisible-sum-pairs/problem?isFullScreen=true)

**Solution**: [easy/divisible_sum_pairs.rs](https://github.com/fabi200123/HackerRank-problems/blob/master/easy/divisible_sum_pairs.rs)

**NOTE**: The problem requires me to output the number of pairs from an array that are divisible with k. For this, I create a `for` `i` from `0` to `n-1` and another one from `i` to `n`, thus increasing the counter for every `(i, j)` pair such that `(ar[i] + ar[j])` is divided by `k`.

14. [Find Digits](https://www.hackerrank.com/challenges/find-digits/problem?isFullScreen=true)

**Solution**: [easy/find_digits.rs](https://github.com/fabi200123/HackerRank-problems/blob/master/easy/find_digits.rs)

**NOTE**: The problem requires me to check how many of the digits of a number divide it. For this, I copy the number to an `aux` mutable variable. Next, while `aux` is not 0, I get the last digit of aux, check if the number is divided with it (increase a counter if it is), then update aux to be `aux / 10` (removing the last digit from the list of digits to be checked - `aux`). This goes on, until there are no digits left in `aux`

### Medium Problems

1. [Extra Long Factorials](https://www.hackerrank.com/challenges/extra-long-factorials/problem?isFullScreen=true)

**Solution**: [medium/extra_long_factorials.rs](https://github.com/fabi200123/HackerRank-problems/blob/master/medium/extra_long_factorials.rs)

**NOTE**: The problem requires me to do the factorial of the integer n (`1 <= n <= 100`). For this, I use the BigInt from num library to store the result and I do a `for 1..=n` and do the product of all elements.

2. [Encryption](https://www.hackerrank.com/challenges/encryption/problem?isFullScreen=true)

**Solution**: [medium/encryption.rs](https://github.com/fabi200123/HackerRank-problems/blob/master/medium/encryption.rs)

**NOTE**: The problem requires me to transform a string into an encrypted script. For this, I first need to remove the spaces from the message. Then I calculate the sqrt of the length of the string, based on floor/ceil I calculate the rows/columns. Based on the rows and columns and iterating step by step over the characters, I recreate the words for the encrypted message, which I return with spaces between these words.

3. [Fibonacci Modified](https://www.hackerrank.com/challenges/fibonacci-modified/problem?isFullScreen=true)

**Solution**: [medium/fibbonaci_modified.rs](https://github.com/fabi200123/HackerRank-problems/blob/master/medium/fibbonaci_modified.rs)

**NOTE**: The problem requires me to do the fibbonaci with a modified formula. The solution is pretty straight-forward, the only issue is the storage related part. In order to store the values, I have used the `BigInt` from `num` library, thus having an easy solutions, based on a for loop starting from the 3rd number in sequence and calculating to the n-th.

### Hard Problems
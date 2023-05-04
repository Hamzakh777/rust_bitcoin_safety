# Demonstrate Bitcoin safety

## Requirements
1) Given an array A of 16 numbers (settable) A=a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p ( the numbers are consecutive)
 
2) Calculate every possible combination without repetition of of 9-10-11-12 elements obtained from A. [1,2,3,4,5,6,7,8,9]
3) Once obtained every possible combination without repetition of 9-10-11-12 element from A, calculate every possible permutation with repetition of this intervals in a new array’s of X length (settable). X = 13, [1,1,3,4,5,6,7,8,9,9,9,9,9,9,9]

3.1) I should be able to tell to the script which will be the first array obtained from permutation with reputation (for example, cancel every permutation that starts with x,y,z and consider only the only one that starts with d).  This will be valid also for the last element of array obtained from permutation with repetition. 
 
4) Once the script starts to calculate permutation, convert automatically the elements obtained in the array in binary code, for example: 
 
4.1) From the array A script choose an interval of 10 elements (2,3,4,5,6,7,8,9,10,11) 
4.2) I chose a length of 18 elements for the calculation of permutation with reputation, so, one of the permutation with repetition could be (2,4,3,5,8,9,10,11,2,5,4,4,7,8,9,9,9,2) 
4.3) (Translate this code in binary code, keepin in mind that binary code should be of 4 length,and only the last one number of the interval could be septable from 2 to 4 length so for example we’ll have:  
Output: 0010 0100 0011 0101 1000 1001 1010 1011 0010 0101 0100 0100 0111 1000,1001 1001 1001 10 
 
5) Once you have this binary code, convert it in hexadecimal value. 
6) Obtain the relative public key compressed. 
7) SHA256 of public key Compressed. 
8) Ripemd160 
9) I’ll give on the input an hash160. If at the step 8, the script will find the same hash160 and will compare to it, it will stop cycle and communicate to me: 
- Point 5 found 
- Point 6 found 
- Point 8 found.

## Example
1) Given an array of A= (0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15) 
2) Set “condition” for first digit of the permutation with repetition obtained x >=7 
3) Set “condition” for the last digit of the permutation with repetition obtained y <=3 
4) Set “condition” for the last digit binary length in conversion phase= 1,2,3,4? (We choose in this example 2) 
5) Set an hash160 to be compared: 24eb23f3cf0e14458f07ef0ce9d1e09c5e25e00d 
6) Calculate every possible combination without repetition of 9, 10, 11, 12 elements taken from the array A: 
for 9 elements you’ll receive this combination: 
- 0 1 2 3 4 5 6 7 8 
- 0 1 2 3 4 5 6 7 9 
 -0 1 2 3 4 5 6 7 10 
ecc. for 9 elements you have 11440 combination possible. 
 
for 10 elements you’ll receive this combination: 
- 0 1 2 3 4 5 6 7 8 9 
- 0 1 2 3 4 5 6 7 8 10 
- 0 1 2 3 4 5 6 7 8 11 
ecc. for 10 elements you have 8008 combination possible. 
for 11 elements you’ll receive this combination: 
0 1 2 3 4 5 6 7 8 9 10  
0 1 2 3 4 5 6 7 8 9 11  
0 1 2 3 4 5 6 7 8 9 12 
 
for 11 elements you have more or less 4358 combination possible. 
 
for 12 elements you’ll receive this combination: 
0 1 2 3 4 5 6 7 8 9 10 11  
0 1 2 3 4 5 6 7 8 9 10 12  
0 1 2 3 4 5 6 7 8 9 10 13 

for 11 elements you have more or less 1860 combination possible. 
 
7) Given a length to be settled (10,11,12,18, ecc.) calculate every possible permutation with repetition of this interval obtained. We should be able to tell to the script that first element and last elements should be less or equal to y. So for example, let’s choose as first element x=7 and as last element y=3 
For example for 9 elements we choose this interval “0 1 2 3 4 5 6 7 8” 
 
So, possible permutation with repetition for an array with length 13 could be 
 
(0,1,2,3,4,5,6,7,8,1,1,1,6) (0,1,2,3,3,5,6,7,8,4,8,7,6) ecc. ecc. 
 
But we have settled rules that x>=7 and y<=3 ! So, we’ll have 
 
(7, 0,1,2,3,4,5,6,7,8,1,1,3) (7,5,1,2,3,4,2,6,7,8,1,1,3) ecc ecc. 
 
8) convert automatically the elements obtained in the array in binary code (maximum 4 digits for each elements, for the last one it could be between 0-4, so, it should be settled at the start of script, for example: 
 
(7, 0,1,2,3,4,5,6,7,8,1,1,3) = 0111 0000 0001 0010 0011 0100 0101 0110 0111 1000 0001 0001 11 
 
9) Take this binary code and convert it into hex code: 
0111 0000 0001 0010 0011 0100 0101 0110 0111 1000 0001 0001 11 = 1c048d159e047 (consider this as private key)

10) Calculate public key compressed of “1c048d159e047" 
02c81a115702c33f88b7bba091116b6ba7e9916267f26dd241351553c5a34a872f 
 
11) Execute sha256 
7ceb4f18dfcdbdb3552b24d151efb5ba3e834680ff0bd108daa5c0d2ec50393f 
 
12) Execute ripemd160 

13) once found that is equal to the script, stop the script and print the has160, and the key found (print step 9 and step 12)


## Flow
1. We can give the script just the start number for array A and we'll automatically generate the rest
   1. give it like start = 4
   2. the script will automatically 4 till 20 
2. When we start the script we should give it X.
3. Set an hash160 to be compared: 24eb23f3cf0e14458f07ef0ce9d1e09c5e25e00d 
4. X is the array's length 
5. x is the first number in the array
6. y is the last number
7. give the script how many bits to take of the last number
8. limit the numbers in Array A to max 4 bits
9. do some benchmarks -> how many combinations we can calculate per second

when we generate every pertumutation with repetition, we should keep in mind that it must have every single number from the permutation without repetition 

## In short
Given a hash160 and a list of number, find the numbers permutations with repetitions that will result in the given hash160
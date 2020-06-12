# Exercises

👉 [Click to see the solutions](.).

## Exercise 1

You have given a list of integers.

1. Use a vector and return the mean (the average value).
2. Return the median (_when sorted, the value in the middle position_).
3. Return the mode (_the value that occurs most often; a hash map will be helpful here_) of the list.

## Exercise 2

Convert strings to pig latin (_Keep in mind the details about UTF-8 encoding!_).

-> The first consonant of each word is moved to the end of the word and "ay" is added:
   
    "first" becomes "irst-fay"

-> Words that start with a vowel have "hay" added to the end instead:
    
    "apple" becomes "apple-hay"

## Exercise 3

Using a `hash map` and `vectors`, create a text interface to allow a user to add employee names to a department in a company.

* For example, "Add Sally to Engineering" or "Add Amir to Sales."
* Then let the user retrieve a list of all people in a department
* Or all people in the company by department, sorted alphabetically.

---
_Exercises are copied from the original Rust book [here](https://doc.rust-lang.org/nightly/book/ch08-03-hash-maps.html)._
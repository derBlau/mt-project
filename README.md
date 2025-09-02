# Multithreading Assignment

## Objective

Create a multithreaded program that is able to take lines from stdin, which
contains a path to a file, and perform three operations:

- [x] replace words that start with a given character with another given word
      and print the result to stdout
- [x] replace each occurrence of a given character with another combination of
      characters and print the result to stdout
- [] make other adjustments to the text and print the result to stdout

## Other design aspects

- [] the program is able to take terminal args as input
- [] the program is able to spawn as many worker threads as the system allows
- [] the workload should be equally distributed amongst all threads: 1/n, where
  n is the number of worker threads
- [] worker threads use an mpsc queue to send the data back to the main thread
- [x] the program must include tests

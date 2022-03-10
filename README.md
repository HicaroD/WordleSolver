# WordleSolver
:green_square: An algorithm to solve Wordle puzzles.

### Explanations
With the Wordle dictionary on hands, I could write a fairly simple algorithm to filter the entire list of words. I didn't measure,
but I'm pretty sure this algorithm is very inneficient and there are plenty of better implementations of a Wordle Solver.

Maybe in the near future, I could improve my algorithm precision by taking advantage of the common word list and the letter frequency list, so I can show more precise and reliable results.

Additionally, I need to deal with cases when the user inserts a word that contains repetead characters, but the result only has one of these characters, that will make my algorithm confuses because it will basically flag one of these letters as `NotFound` and then eliminate every occurrence of that letter in any word.

## License
This project is licensed under the [MIT](./LICENSE) license.

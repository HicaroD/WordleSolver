# WordleSolver
:green_square: A simple algorithm to solve Wordle puzzles.

### Installation

   1. Clone the project and go to the folder
    
   ```bash
   git clone https://github.com/HicaroD/WordleSolver && cd WordleSolver
   ```
    
   2. Run the project
    
   ```bash
   cargo run --release
   ```

### Usage
   In order to represent the pattern below in my program, you can use the following keywords: `B`(Black), `G`(Green) and `O`(Orange).

![pattern](Images/pattern_example.png)

The input of the program should be `c-O r-B a-B t-B e-B`. You need to associate each word with your status / color and try to use to use the outputs of the program for upcoming words. Hopefully, you get good patterns to make the list of words as short as possible.

### Explanations

   With the Wordle dictionary on hands, I could write a fairly simple algorithm to filter the entire list of words. I didn't measure, but I'm pretty sure this algorithm is very inneficient and certainly there are plenty of better implementations of a Wordle Solver. 
   In order to take a little bit more advantage, try to use some good opening words, such as "**crate**",, "**soare**" or "**crane**". If you want to know about information theory in the context of Wordle solving algorithms, watch [this](https://www.youtube.com/watch?v=v68zYyaEmEA).

   Maybe in the near future, I could improve my algorithm precision by taking advantage of the list of common word and letter frequency, so I can show more precise and reliable results.

### License

   This project is licensed under the [MIT](./LICENSE) license.

# How to run 
Install [Rust](https://www.rust-lang.org/tools/install) 

In project root run `cargo run transacciones.txt 2 0.50`  
Argument 1 is the text file that contains the data set to be processed. .txt must be in project root.  
Argument 2 is integer value that represents minimum support.  
Argument 3 is a decimal value between 0 and 1 that represents minimum confidence percentage.  


# Example run
Output example for this command: `cargo run transacciones.txt 2 0.50`
The sample data set `transacciones.txt` provided in the repo is used.

Output:
```
Item set: ["A", "B", "C", "D", "E", "F"]
Number of Items:6
Number of transactions: 5
A => support = 3
B => support = 4
C => support = 4
D => support = 2
E => support = 3
Frequent Itemset: 2
A B => support = 2
A C => support = 3
A D => support = 2
B C => support = 3
B E => support = 3
C D => support = 2
C E => support = 2
Frequent Itemset: 3
A B C => support = 2
A C D => support = 2
B C E => support = 2
Frequent Itemset: 4
Frequent Itemset: 5

Association Rules for Frequent Itemset
A -> B = Confidence = 66.66667 and Support = 2
B -> A = Confidence = 50 and Support = 2
A -> C = Confidence = 100 and Support = 3
C -> A = Confidence = 75 and Support = 3
A -> D = Confidence = 66.66667 and Support = 2
D -> A = Confidence = 100 and Support = 2
B -> C = Confidence = 75 and Support = 3
C -> B = Confidence = 75 and Support = 3
B -> E = Confidence = 75 and Support = 3
E -> B = Confidence = 100 and Support = 3
C -> D = Confidence = 50 and Support = 2
D -> C = Confidence = 100 and Support = 2
C -> E = Confidence = 50 and Support = 2
E -> C = Confidence = 66.66667 and Support = 2
A ->  B C = Comfidemce = 66.66667 and Support = 2
B C -> A = Comfidemce = 66.66667 and Support = 2
A B ->  C = Comfidemce = 100 and Support = 2
C -> A B = Comfidemce = 50 and Support = 2
A ->  C D = Comfidemce = 66.66667 and Support = 2
C D -> A = Comfidemce = 100 and Support = 2
A C ->  D = Comfidemce = 66.66667 and Support = 2
D -> A C = Comfidemce = 100 and Support = 2
B ->  C E = Comfidemce = 50 and Support = 2
C E -> B = Comfidemce = 100 and Support = 2
B C ->  E = Comfidemce = 66.66667 and Support = 2
E -> B C = Comfidemce = 66.66667 and Support = 2
```

# Rust-Poker-Game
Poker game created in Rust which determines winning hand given a deck of 10 cards

Code can be run by addding the following code at the end of the file while changing the 1..10 to whatever numbers you desire
```
fn main() {
  let perm:[u32;10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
  let winner:[&str;5] = deal(perm); 
}
``` 

the 10 values inside Poker.deal range from 1-52 where 1 is the Ace of Clubs and 52 is the King of Spades going in order from A-K and by suits Clubs, Diamonds, Hearts, and Spades.

/*
Seyon Rajagopal

CPS 506 - Rust Paker Game Assignment

*/

//
fn deal(deck: [u32;10]) -> [&'static str; 5]{
  //split deck into 2 hands
  let hand1 = [deck[0],deck[2],deck[4],deck[6],deck[8]];
  let hand2 = [deck[1],deck[3],deck[5],deck[7],deck[9]];

  //deck of cards with 14 represnted as 1 (used 14 used for sorting)
  let deck1 = [(14,'C'),(2,'C'),(3,'C'),(4,'C'),(5,'C'),(6,'C'),(7,'C'),(8,'C'),(9,'C'),(10,'C'),(11,'C'),(12,'C'),(13,'C'),(14,'D'),(2,'D'),(3,'D'),(4,'D'),(5,'D'),(6,'D'),(7,'D'),(8,'D'),(9,'D'),(10,'D'),(11,'D'),(12,'D'),(13,'D'),(14,'H'),(2,'H'),(3,'H'),(4,'H'),(5,'H'),(6,'H'),(7,'H'),(8,'H'),(9,'H'),(10,'H'),(11,'H'),(12,'H'),(13,'H'),(14,'S'),(2,'S'),(3,'S'),(4,'S'),(5,'S'),(6,'S'),(7,'S'),(8,'S'),(9,'S'),(10,'S'),(11,'S'),(12,'S'),(13,'S')];

  //deck of cards with 1 represnted as 1 (used to convert back for final output)
  let deck2 = [(1,'C'),(2,'C'),(3,'C'),(4,'C'),(5,'C'),(6,'C'),(7,'C'),(8,'C'),(9,'C'),(10,'C'),(11,'C'),(12,'C'),(13,'C'),(1,'D'),(2,'D'),(3,'D'),(4,'D'),(5,'D'),(6,'D'),(7,'D'),(8,'D'),(9,'D'),(10,'D'),(11,'D'),(12,'D'),(13,'D'),(1,'H'),(2,'H'),(3,'H'),(4,'H'),(5,'H'),(6,'H'),(7,'H'),(8,'H'),(9,'H'),(10,'H'),(11,'H'),(12,'H'),(13,'H'),(1,'S'),(2,'S'),(3,'S'),(4,'S'),(5,'S'),(6,'S'),(7,'S'),(8,'S'),(9,'S'),(10,'S'),(11,'S'),(12,'S'),(13,'S')];

  //choose deck of cards based on integer input and sort
  let mut converted_hand1 = [deck1[(hand1[0]-1) as usize], deck1[(hand1[1]-1) as usize], deck1[(hand1[2]-1) as usize], deck1[(hand1[3]-1) as usize], deck1[(hand1[4]-1) as usize]];
  let mut converted_hand2 = [deck1[(hand2[0]-1) as usize], deck1[(hand2[1]-1) as usize], deck1[(hand2[2]-1) as usize], deck1[(hand2[3]-1) as usize], deck1[(hand2[4]-1) as usize]];
  converted_hand1.sort_by(|a,b| a.cmp(b));
  converted_hand2.sort_by(|a,b| a.cmp(b));

  //choose deck of cards based on integer input and sort
  let mut converted_ohand1 = [deck2[(hand1[0]-1) as usize], deck2[(hand1[1]-1) as usize], deck2[(hand1[2]-1) as usize], deck2[(hand1[3]-1) as usize], deck2[(hand1[4]-1) as usize]];
  let mut converted_ohand2 = [deck2[(hand2[0]-1) as usize], deck2[(hand2[1]-1) as usize], deck2[(hand2[2]-1) as usize], deck2[(hand2[3]-1) as usize], deck2[(hand2[4]-1) as usize]];
  converted_ohand1.sort_by(|a,b| a.cmp(b));
  converted_ohand2.sort_by(|a,b| a.cmp(b));

  //choose winner by comparing tuples and return the output as array of strings
  let mut winner = if evaluate(converted_hand1) > evaluate(converted_hand2) {
    flatten(&converted_ohand1)
  }else {
    flatten(&converted_ohand2)
  };

  //sort the array of strings
  winner.sort();
  winner
}

//determine hand type using pattern matching
fn evaluate(hand: [(u32,char);5]) -> (u32,u32,u32,u32,u32,u32,u32) {
  match hand {
    //Royal Flush
    [(x1, y1),(x2, y2),(x3, y3),(x4, y4),(x5, y5)] if (x1 == 10) && (x2 == 11) && (x3 == 12) && (x4 == 13) && (x5 == 14 && y1 ==y2 && y2 == y3 && y3 == y4 && y4 == y5) => (10,y1 as u32,0,0,0,0,0 ),

    // Straight Flush
    [(x1, y1),(x2, y2),(x3, y3),(x4, y4),(x5, y5)] if (x1 == 2) && (x2 == 3) && (x3 == 4) && (x4 == 5) && (x5 == 14 && y1 == y2 && y2 == y3 && y3 == y4 && y4 == y5) => (9,5,y1 as u32,0,0,0,0),
    [(x1, y1),(x2, y2),(x3, y3),(x4, y4),(x5, y5)] if (x1+1 == x2) && (x2+1 == x3) && (x3+1 == x4) && (x4+1 == x5) && (y1 == y2 && y2 == y3 && y3 == y4 && y4 == y5 ) => (9,x5,y1 as u32,0,0,0,0), 

    //Four of a kind
    [(x1, _y1),(x2, _y2),(x3, _y3),(x4, y4),(x5, _y5)] if x1 == x2 && x2 == x3 && x3 == x4 => (8,x1,x5,y4 as u32,0,0,0),
    [(x1, _y1),(x2, _y2),(x3, _y3),(x4, _y4),(x5, y5)] if x2 == x3 && x3 == x4 && x4 == x5 => (8,x2,x1,y5 as u32,0,0,0),

    // Full House
    [(x1, _y1),(x2, _y2),(x3, y3),(x4, _y4),(x5, _y5)] if x1 == x2 && x2 == x3 && x4 == x5 => (7,x1,x4,y3 as u32,0,0,0),
    [(x1, _y1),(x2, _y2),(x3, _y3),(x4, _y4),(x5, y5)] if x3 == x4 && x4 == x5 && x1 == x2 => (7,x5,x2,y5 as u32,0,0,0), 

    //Flush
    [(x1, y1),(x2, y2),(x3, y3),(x4, y4),(x5, y5)] if y1 == y2 && y2 == y3 && y3 == y4 && y4 == y5 => (6,x5,x4,x3,x2,x1,y5 as u32),  

    //Straight
    [(x1, _y1),(x2, _y2),(x3, _y3),(x4, _y4),(x5, y5)] if x1+1 == x2 && x2+1 == x3 && x3+1 == x4 && x4+1 == x5 => (5,x5,y5 as u32,0,0,0,0), 
    [(x1, _y1),(x2, _y2),(x3, _y3),(x4, _y4),(x5, y5)] if x1 == 2 && x2 == 3 && x3 == 4 && x4 == 5 && x5 == 14 => (5,5,y5 as u32,0,0,0,0),

    //Three of a Kind
    [(x1, _y1),(x2, _y2),(x3, y3),(x4, _y4),(x5, _y5)] if x1 == x2 && x2 == x3 => (4,x1,x4,x5,y3 as u32,0,0),
    [(x1, _y1),(x2, _y2),(x3, _y3),(x4, y4),(x5, _y5)] if x2 == x3 && x3 == x4 => (4,x2,x5,x1,y4 as u32,0,0),
    [(x1, _y1),(x2, _y2),(x3, _y3),(x4, _y4),(x5, y5)] if x3 == x4 && x4 == x5 => (4,x3,x2,x1,y5 as u32,0,0),

    //Two pair
    [(x1, _y1),(x2, _y2),(x3, _y3),(x4, y4),(x5, _y5)] if x1 == x2 && x3 == x4 => (3,x3,x1,x5,y4 as u32,0,0),
    [(x1, _y1),(x2, _y2),(x3, _y3),(x4, _y4),(x5, y5)] if x1 == x2 && x4 == x5 => (3,x4,x1,x3,y5 as u32,0,0),
    [(x1, _y1),(x2, _y2),(x3, _y3),(x4, _y4),(x5, y5)] if x2 == x3 && x4 == x5 => (3,x4,x2,x1,y5 as u32,0,0),

    //Pair
    [(x1, _y1),(x2, y2),(x3, _y3),(x4, _y4),(x5, _y5)] if x1 == x2 => (2,x1,x5,x4,x3,y2 as u32,0),
    [(x1, _y1),(x2, _y2),(x3, y3),(x4, _y4),(x5, _y5)] if x2 == x3 => (2,x2,x5,x4,x1,y3 as u32,0),
    [(x1, _y1),(x2, _y2),(x3, _y3),(x4, y4),(x5, _y5)] if x3 == x4 => (2,x3,x5,x2,x1,y4 as u32,0),
    [(x1, _y1),(x2, _y2),(x3, _y3),(x4, _y4),(x5, y5)] if x4 == x5 => (2,x4,x3,x2,x1,y5 as u32,0),

    //High Card
    [(x1, _y1),(x2, _y2),(x3, _y3),(x4, _y4),(x5, y5)] => (1,x5,x4,x3,x2,x1,y5 as u32),

  }
}
//Make the return type to array of static str.
fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

//coverts a list of tuples to an array of static str
fn flatten(data: &[(u32, char)]) -> [&'static str; 5] {
    let mut result:[&str;5] = Default::default();
    for (i, &(a, b)) in data.iter().enumerate() {
        result[i] = string_to_static_str([a.to_string(), b.to_string()].join(""));
    }
    result
}

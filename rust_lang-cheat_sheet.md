## Control Flow

### Range

#### Normal range (excludes the end)
``` rust
let y = 0..10;
```

#### Inclusive range
``` rust
let y = 0..=9;
```


#### Looping over iterators
```rust
for x in collection {
}	
```
```rust
for x in 1..100 {
}
```


#### `break` with label
```rust
'outer: loop {
    while true {
        break 'outer;
    }
}
```

#### `break` with returned value
```rust
let result = loop {
    if b > 10 {
        break b;
    }
    let c = a + b;
    a = b;
    b = c;
};
```

### "switch ~ case"

#### Example 1
```rust
let x = 1;

match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    4 => println!("four"),
    5 => println!("five"),
    _ => println!("something else"),
}
```


#### Example 2
```rust
let x = 1;

let x = 9;
let message = match x {
    0 | 1  => "not many",
    2 ..= 9 => "a few",
    _      => "lots"
};
```

## Data Structure

### Linear | Sequence: arrays, queues, stacks
* Use a `Vec` when: you want a stack | resizable array | heap-like array
* Use a `VecDeque` when: you want a queue | double-ended queue |  a Vec that supports efficient insertion at both ends of the sequence
** Note: plz remember to `use std::collections::VecDeque;`
* Use a `LinkedList` when: You are absolutely certain you really, truly, want a doubly linked list.

```
.push_front(x) | .front() | .pop_front()
.push_back(x)  | .back()  | .pop_back()
```

```rust
use std::collections::VecDeque;

let vec = vec![1, 2, 3, 4];
for x in vec.iter() {
   println!("vec contained {x:?}");
}

let mut vec1 = vec![1, 2, 3, 4];
for x in vec1.iter_mut() {
   *x += 1;
}

let vec2 = vec![10, 20, 30, 40];
vec1.extend(vec2);


let vec3 = [1, 2, 3, 4];
let vecdeque1: VecDeque<_> = vec3.into_iter().collect();
for x in vec3.iter().rev() {
   println!("vec contained {x:?}");
}


let arr = vec![100; size];
let mut vecdeque2 = VecDeque::new();
arr.clone()
    .into_iter()
    .for_each(|element| queue.push_back(element));

for _ in 0..arr.len() {
    vecdeque2.pop_front();
}

let vecdeque3: VecDeque<i32> = VecDeque::with_capacity(10);

let vecdeque4 = VecDeque::from([-1, 0, 1]);

```

### HashMap, HashSet
```rust
use std::collections::HashMap;

let mut book_reviews = HashMap::new();
book_reviews.insert(
    "Adventures of Huckleberry Finn".to_string(),
    "My favorite book.".to_string(),
);
book_reviews.insert(
    "Grimms' Fairy Tales".to_string(),
    "Masterpiece.".to_string(),
);

if !book_reviews.contains_key("Les Misérables") {
    println!("We've got {} reviews, but Les Misérables ain't one.",
             book_reviews.len());
}

// oops, this review has a lot of spelling mistakes, let's delete it.
book_reviews.remove("The Adventures of Sherlock Holmes");

// Look up the values associated with some keys.
let to_find = ["Pride and Prejudice", "Alice's Adventure in Wonderland"];
for &book in &to_find {
    match book_reviews.get(book) {
        Some(review) => println!("{book}: {review}"),
        None => println!("{book} is unreviewed.")
    }
}

// A HashMap with a known list of items can be initialized from an array:
let solar_distance = HashMap::from([
    ("Mercury", 0.4),
    ("Venus", 0.7),
    ("Earth", 1.0),
    ("Mars", 1.5),
]);

// read a key, if the key is empty, insert it with a default value
let mut player_stats = HashMap::new();
player_stats.entry("health").or_insert(100);

// modify an entry before an insert with in-place mutation
player_stats.entry("mana").and_modify(|mana| *mana += 200).or_insert(100);

```

### map / reduce / collect / apply 
#### char counter for string
```rust
let text = String::from("hello from rust!");
let result = "abcdefghijklmnopqrstuvwxyz"
    .chars()
    .map(|c| (c, text.matches(c).count()))
    .collect::<std::collections::HashMap<_, _>>();
println!("{:?}", result);
 ```

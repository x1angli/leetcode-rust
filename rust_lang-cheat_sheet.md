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

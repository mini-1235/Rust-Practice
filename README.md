# Hw3 Three exercises

## Exercise 1

Implement hash_map macro
Sample case

```
let map = hash_map!{
        "one" => 1,
        "two" => 2,
        "three" => 3
    };
srintln!("{:?}", map);
```

Output

```
{"one": 1, "two": 2, "three": 3}
```

## Exercise 2

Implement MyRc, which has similar functionality as Rc

Sample case

```
let five = MyRc::new(5);
let five2 = MyRc::clone(&five);
println!("{}", *five);
println!("{}", MyRc::strong_count(&five2));
```

Output

```
5
2
```

## Exercise 3

Implement a stack, which supports push and pop, use RefCell

## How to run

```
cd hw3
cargo run --bin <bin-name>
```

bin-name: exercise1, exercise2, exercise3
![Alt text](image.png)

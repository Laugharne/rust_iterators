### Rust

```rust
fn main() {
    let vec_str: Vec<&str> = vec!["A", "B", "C"];
    let mut vec_str_mut: Vec<&str> = vec!["A", "B", "C"];

    println!("\n> iter()");
    vec_str.iter().for_each(|value: &&str| {
        println!("{}", value);
    });

    println!("\n> iter().enumerate()");
    vec_str.iter().enumerate().for_each(|(index,value)| {
        println!("{}\t{}", index, value);
    });

    println!("\n> iter_mut()");
    vec_str_mut.iter_mut().for_each(|value: &mut &str| {
        println!("{}", value);
    });

    println!("\n> iter_mut().enumerate()");
    vec_str_mut.iter_mut().enumerate().for_each(|(index,value)| {
        println!("{}\t{}", index, value);
    });

    println!("\n> vec_str");
    println!("{:?}",vec_str);
    
    println!("\n> vec_str.iter()");
    println!("{:?}",vec_str.iter());
    println!("{:?}",vec_str.iter().next());
    
    println!("\n> vec_str.iter().enumerate()");
    println!("{:?}",vec_str.iter().enumerate());
    println!("{:?}",vec_str.iter().enumerate().next());

    println!("\n> vec_str_mut.iter_mut().map()");
    let r: Vec<_> = vec_str_mut.iter_mut().map(|x: &mut &str| {
        "X"
    }).collect();
    println!("r = {:?}",r);

    println!("\n> vec_str_mut.iter_mut().for_each()");
    vec_str_mut.iter_mut().for_each( |x: &mut &str| {
        *x = "Y";
    });
    println!("{:?}",vec_str_mut);

    println!("\n> vec_str_mut.iter_mut().for_each()");
    for x in vec_str_mut.iter_mut() {
        *x = "Z";
    }
    println!("{:?}",vec_str_mut);

    let v1: Vec<i32> = vec![1,2,3];
    // map takes in a closure and creates an iterator wich close the closure over each elements in a sequence
    let v2: Vec<_> = v1.iter().map(|x: &i32| x+1).collect();
    println!("\nv1 = {:?}",v1);
    println!("\n> collect()");
    println!("v2 = {:?}",v2);

    // Create our own iterators
    // https://www.youtube.com/watch?v=4GcKrj4By8k&t=522s

    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Counter {
            Counter{ count: 0}
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    println!();

    let mut counter: Counter = Counter::new();

    //let _ = counter.next();
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());

    let sum: u32 =
        Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a*b)
            .filter(|x| x % 3 == 0)
            .sum();

            println!("\n{:?}", sum);

}
```

### Output

```bash
❯ cargo r
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/iterators`

> iter()
A
B
C

> iter().enumerate()
0       A
1       B
2       C

> iter_mut()
A
B
C

> iter_mut().enumerate()
0       A
1       B
2       C

> vec_str
["A", "B", "C"]

> vec_str.iter()
Iter(["A", "B", "C"])
Some("A")

> vec_str.iter().enumerate()
Enumerate { iter: Iter(["A", "B", "C"]), count: 0 }
Some((0, "A"))

> vec_str_mut.iter_mut().map()
r = ["X", "X", "X"]

> vec_str_mut.iter_mut().for_each()
["Y", "Y", "Y"]

> vec_str_mut.iter_mut().for_each()
["Z", "Z", "Z"]

v1 = [1, 2, 3]

> collect()
v2 = [2, 3, 4]

Some(1)
Some(2)
Some(3)
Some(4)
Some(5)
None

18
```

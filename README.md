
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
        Counter::new()                   // Counter
            .zip(Counter::new().skip(1)) // impl Iterator<Item = (u32,u32)>
            .map(|(a, b)| a*b)           // impl Iterator<Item = u32>
            .filter(|x| x % 3 == 0)      // impl Iterator<Item = u32>
            .sum();

            println!("\n{:?}", sum);

}
```

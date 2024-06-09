fn main() {
	let vec_str = vec!["A", "B", "C"];
	let mut vec_str_mut = vec!["A", "B", "C"];

	println!("\n> iter()");
	vec_str.iter().for_each(|value| {
		println!("{}", value);
	});

	println!("\n> iter().enumerate()");
	vec_str.iter().enumerate().for_each(|(index,value)| {
		println!("{}\t{}", index, value);
	});

	println!("\n> iter_mut()");
	vec_str_mut.iter_mut().for_each(|value| {
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


	let v1 = vec![1,2,3];
	// map takes in a closure and creates an iterator wich close the closure over each elements in a sequence
	let v2: Vec<_> = v1.iter().map(|x| x+1).collect();
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

	let mut counter = Counter::new();

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

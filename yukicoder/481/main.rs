fn getline() -> String{
	let mut __ret=String::new();
	std::io::stdin().read_line(&mut __ret).ok();
	return __ret;
}

fn main() {
	let mut bs: Vec<usize> = getline().trim().split(' ').map(|x| x.parse().unwrap()).collect();

	bs.sort();

	let mut res = 10;
	for (i, b) in bs.into_iter().enumerate() {
		if b != i + 1 {
			res = i + 1;
			break;
		}
	}

	println!("{}", res);
}

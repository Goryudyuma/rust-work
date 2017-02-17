fn getline() -> String{
	let mut __ret=String::new();
	std::io::stdin().read_line(&mut __ret).ok();
	return __ret;
}

fn main(){
	getline();
	let ls: Vec<usize> = getline().trim().split(' ').map(|x| x.parse().unwrap()).collect();

	let mut count: Vec<usize> = vec![0; 6];

	for (_, l) in ls.into_iter().enumerate() {
		count[l-1] += 1;	
	}
	let mut num = 0;
	let mut ans = 0;
	for (i, c) in count.into_iter().enumerate() {
		if num <= c {
			ans = i + 1;
			num = c;
		}
	}
	println!("{}", ans);
}

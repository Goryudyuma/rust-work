fn getline() -> String{
	let mut __ret=String::new();
	std::io::stdin().read_line(&mut __ret).ok();
	return __ret;
}

fn main(){
	let s = getline();
	let bstr:Vec<&str>=s.trim().split(' ').collect();
	let mut bint:[usize; 9] = [0; 9];
	let mut i = 0;
	while i < 9 {
		bint[i] = bstr[i].parse().unwrap();
		i += 1;
	}
	bint.sort();
	i = 1;
	while i <= 10 {
		if i == 10 {
			println!("{}", 10);
			break;
		} else if bint[i-1] != i {
			println!("{}", i);
			break;
		}
		i += 1;
	}
}

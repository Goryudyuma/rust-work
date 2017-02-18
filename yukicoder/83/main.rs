fn getline() -> String{
	let mut __ret=String::new();
	std::io::stdin().read_line(&mut __ret).ok();
	return __ret;
}

fn main(){
	let mut num:i64 = getline().trim().parse().unwrap();
	if num % 2 == 1 {
		num -= 3;
		print!("7");
	}
	while num != 0 {
		print!("1");
		num -= 2;
	}
	println!("");
}

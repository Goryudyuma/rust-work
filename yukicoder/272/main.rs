fn getline() -> String{
	let mut __ret=String::new();
	std::io::stdin().read_line(&mut __ret).ok();
	return __ret;
}

fn main(){
	let x: i64 = getline().trim().parse().unwrap();
	println!("{}", if x== 1 {0} else {1});
}


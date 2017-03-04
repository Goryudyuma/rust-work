fn getline() -> String{
	let mut __ret=String::new();
	std::io::stdin().read_line(&mut __ret).ok();
	return __ret;
}

fn main(){
	let n :i64 = getline().trim().parse().unwrap();
	println!("{}", if n % 2 == 0 {n / 2} else {n});
}

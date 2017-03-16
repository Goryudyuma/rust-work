fn getline() -> String{
	let mut __ret=String::new();
	std::io::stdin().read_line(&mut __ret).ok();
	return __ret;
}

fn main(){
	let ls: Vec<i64> = getline().trim().split(' ').map(|x| x.parse().unwrap()).collect();

	println!("{}", ((ls[0] / 5) as i64 * ls[1]) as f64 / (ls[2] as f64));
}

fn getline() -> String{
	let mut __ret=String::new();
	std::io::stdin().read_line(&mut __ret).ok();
	return __ret;
}

fn main(){
	getline();
	let mut ls: Vec<i64> = getline().trim().split(' ').map(|x| x.parse().unwrap()).collect();

	ls.sort();

	println!("{}",
		if ls.len() % 2 == 0 {
			(ls[ls.len()/2] + ls[ls.len()/2-1]) as f64 / 2.0
		} else {
			ls[ls.len()/2] as f64
		}
	);

}

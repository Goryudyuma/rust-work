fn getline() -> String{
	let mut __ret=String::new();
	std::io::stdin().read_line(&mut __ret).ok();
	return __ret;
}

fn main(){
	let ps: Vec<f64> = getline().trim().split(' ').map(|x| x.parse().unwrap()).collect();
	let qs: Vec<f64> = getline().trim().split(' ').map(|x| x.parse().unwrap()).collect();

	println!("{}", ((ps[0] - qs[0]).abs() + ((ps[1] - qs[1]).abs())) / 2 as f64);
}

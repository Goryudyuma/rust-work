fn getline() -> String{
	let mut __ret=String::new();
	std::io::stdin().read_line(&mut __ret).ok();
	return __ret;
}

fn main(){
	let v: Vec<f64> = getline().trim().split(' ').map(|x| x.parse().unwrap()).collect();
	let d: f64 = getline().trim().parse().unwrap();
	let w: f64 = getline().trim().parse().unwrap();
	println!("{}", d / (v[0] + v[1]) * w);
}

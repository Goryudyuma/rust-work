fn getline() -> String{
	let mut __ret=String::new();
	std::io::stdin().read_line(&mut __ret).ok();
	return __ret;
}

fn main(){
	let ls: Vec<usize> = getline().trim().split(' ').map(|x| x.parse().unwrap()).collect();

	println!("{}", (ls[0] - 1) / (ls[1] * 2) * ls[1]);
}

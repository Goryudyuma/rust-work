fn getline() -> String{
	let mut __ret=String::new();
	std::io::stdin().read_line(&mut __ret).ok();
	return __ret;
}

fn main(){
	println!("{}", getline().trim().replace("-", "").chars().count()/3);
}

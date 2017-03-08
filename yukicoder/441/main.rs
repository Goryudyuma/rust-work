fn getline() -> String{
	let mut __ret=String::new();
	std::io::stdin().read_line(&mut __ret).ok();
	return __ret;
}

fn main(){
	let ab: Vec<String> = getline().trim().split(' ').map(|x| x.parse().unwrap()).collect();

	if ab[0] == "2" && ab[1] == "2" || ab[0] == "0" && ab[1] == "0" {
		println!("E");
	} else if ab[0] == "0" || ab[0] == "1" || ab[1] == "0" || ab[1] == "1" {
		println!("S");
	} else {
		println!("P");
	}
}

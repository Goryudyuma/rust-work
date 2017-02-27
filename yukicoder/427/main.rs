fn getline() -> String{
	let mut __ret=String::new();
	std::io::stdin().read_line(&mut __ret).ok();
	return __ret;
}

fn main(){
	let hw: Vec<i64> = getline().trim().split(' ').map(|x| x.parse().unwrap()).collect();
	if hw[0] * 3 == hw[1] * 4 {
		println!("TATE");
	} else {
		println!("YOKO");
	}
}

fn getline() -> String{
	let mut __ret=String::new();
	std::io::stdin().read_line(&mut __ret).ok();
	return __ret;
}

fn main(){
	let ls: Vec<i64> = getline().trim().split(' ').map(|x| x.parse().unwrap()).collect();
	println!("{}{}", if ls[0] == ls[1] {""} else {if ls[0] < ls[1] {"+"} else {""}}, ls[1] - ls[0]);
}

fn getline() -> String{
	let mut __ret=String::new();
	std::io::stdin().read_line(&mut __ret).ok();
	return __ret;
}

fn main(){
	let ab : Vec<i64> = getline().trim().split(' ').map(|x| x.parse().unwrap()).collect();
	if ab[0] < ab[1] {
		println!("{}", ab[1] - 2);
	} else {
		println!("{}", 2000000000 - ab[1] - 1);
	}
}

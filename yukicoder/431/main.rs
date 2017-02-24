fn getline() -> String{
	let mut __ret=String::new();
	std::io::stdin().read_line(&mut __ret).ok();
	return __ret;
}

fn main(){
	let ds :Vec<i32> = getline().trim().split(' ').map(|x| x.parse().unwrap()).collect();
	if ds[3] == 1 || (ds[0] + ds[1] + ds[2] < 2){
		println!("SURVIVED");
	} else {
		println!("DEAD");
	}
}

fn getline() -> String{
	let mut __ret=String::new();
	std::io::stdin().read_line(&mut __ret).ok();
	return __ret;
}

fn main(){
	getline();
	let zs :Vec<i64> = getline().trim().split(' ').map(|x| x.parse().unwrap()).collect();
	println!("{}/{}", zs[zs.len() - 1] ,zs[0]);
}

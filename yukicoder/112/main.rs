fn getline() -> String{
	let mut __ret=String::new();
	std::io::stdin().read_line(&mut __ret).ok();
	return __ret;
}

fn main(){
	let n: i64 = getline().trim().parse().unwrap();
	let asv: Vec<i64> = getline().trim().split(' ').map(|x| x.parse().unwrap()).collect();
	let mut sum:i64 = 0;
	for i in 0..n {
		sum += asv[i as usize];
	}
	sum /= n - 1;
	let kame = (sum - n * 2) / 2;
	println!("{} {}", n - kame, kame);
}

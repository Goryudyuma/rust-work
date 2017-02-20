fn getline() -> String{
	let mut __ret=String::new();
	std::io::stdin().read_line(&mut __ret).ok();
	return __ret;
}

fn main(){
	let t:usize = getline().trim().parse().unwrap();

	for _ in 0..t {
		let s = getline();
		let v:Vec<_> = s.trim().split("").collect();
		let memo: Vec<_> = v.iter().filter(|x| x.len() == 1).collect();
		let mut x: Vec<i64> = memo.iter().map(|x| x.parse().unwrap()).collect();
		for i in 0..x.len() {
			for j in 0..x.len()-i-1 {
				x[j] += x[j+1];

				while x[j] >= 10 {
					let mut d:i64 = 0;
					while x[j] != 0 {
						d += x[j] % 10;
						x[j] /= 10;
					}
					x[j] = d;
				}

			}
		}
		println!("{}", x[0]);
	}
}

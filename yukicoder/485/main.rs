fn getline() -> String{
        let mut __ret=String::new();
		        std::io::stdin().read_line(&mut __ret).ok();
				        return __ret;
						}

fn main(){
	let ab : Vec<f64> = getline().trim().split(' ').map(|x| x.parse().unwrap()).collect();
	if (ab[1] / ab[0]) as i64 as f64 * ab[0] == ab[1] {
		println!("{}", ab[1] / ab[0]);
	} else {
		println!("NO");
	}
}

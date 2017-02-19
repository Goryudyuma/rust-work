fn getline() -> String{
	let mut __ret=String::new();
	std::io::stdin().read_line(&mut __ret).ok();
	return __ret;
}

fn main(){
	let astring = getline();
	let bstring = getline();
	let a:i64 = astring.trim().parse().unwrap_or(-1);
	let b:i64 = bstring.trim().parse().unwrap_or(-1);
	if a==-1 || b == -1 {
		println!("NG");
		return;
	}
	if astring.trim() != a.to_string().trim() || bstring.trim() != b.to_string().trim() {
		println!("NG");
		return;
	}
	if !(0 <= a && a <= 12345 && 0 <= b && b <= 12345) {
		println!("NG");
		return;
	}
	println!("OK");
}


pub fn run() -> String {
     
	let mut c = 1;
	'infin: loop {
		for i in (20..1).step_by(-1) {
			if c*20 % i != 0 {
				c += 1;
				continue 'infin;
			}
		}
		break;		
	} 

	(c*20).to_string()
}


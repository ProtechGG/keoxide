use std::{str, fs::{self, File}, io::{stdin, Read}, process::exit, vec};
use bcrypt;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct ZenzonArray{
	pub d: Vec<Zenzon>
}

impl ZenzonArray{
	pub fn read_vec_zenzon(&mut self, opt: &mut Options) -> ZenzonArray{
		let mut buf = String::from("");
		let _res;
		match opt{
			Options::File(file) => _res = file.read_to_string(&mut buf).expect("ohh"),
			Options::_String(f) => buf = f.to_string()
		};
		
		let mut prevpass = String::from("");
		let mut prev_descript = String::from("");
		let mut is_descript = false;
		let mut data: ZenzonArray = ZenzonArray { d: vec![] };
	
		if !buf.contains(':') && !buf.contains(','){
			return data;
		}
	
	
		for i in buf.chars(){
			match i{
				':' => is_descript=true,
				',' => {
					is_descript=false;
					data.d.push(Zenzon{pass: prevpass.clone(), description: prev_descript.clone()});
					prevpass = String::from("");
					prev_descript = String::from("");
				},
				d => {
					if is_descript{
						prev_descript.push(d);
					}else {
						prevpass.push(d);
					}
				}
			}
		}
		self.d = data.d.clone();
		data
	}
	
	pub fn vec_zenzon_to_string(&mut self) -> String{

		let mut zenzon = String::from("");
		for i in self.clone().d{
			zenzon.push_str(&(i.pass + ":" + &i.description + ","));
		}
		zenzon
	}

}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Zenzon{
	pub pass: String,
	pub description: String
}

pub enum Options{
	File(File),
	_String(String)
}



pub fn new_user(masterpass: &str) -> String{
    println!("You are a new user, huh? Tell me the master password: ");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("ERROR READING INPUT...");
    println!("Confirm master password: ");
    let mut inputc = String::new();
    stdin().read_line(&mut inputc).expect("ERROR READING INPUT...");
    if inputc != input{
        println!("Your password confirmation was wrong, consider retrying...");
        exit(69);
    }
    let _file = read_file(masterpass, true).0;
    let temp_hash = bcrypt::hash_with_salt(input, 6, [69; 16]).unwrap().to_string();
    fs::write(masterpass, &temp_hash).expect("Err..");
    return temp_hash; 
}


// bool -> is created or was read
pub fn  read_file(fname: &str, create: bool) -> (Option<File>, bool){
    let f = fs::File::open(fname);
    match f{
        Ok(f) => (Some(f), false),
        Err(_err) => 
        if create {
                match fs::File::create(fname){
                    Ok(f) => (Some(f), true),
                    Err(err) => {
						eprintln!("{}",err);
						exit(69);
					}
                }
        }else{
			let f = File::open(fname);
			let fna;
			match f {
				Err(_e) => fna = None,
				Ok(fil) => fna = Some(fil),
			}
            return (fna, false);
        }
    }
}


pub fn take_input<'a>(data: &'a mut ZenzonArray, fname: &'a str, hash:&str){
    let input_menu = String::from("0: Show Passwords, 1: Add Password, 2: Remove Password, 3: Quit");
    loop{
        println!("{}",input_menu);
        let mut input = String::new();
        stdin().read_line(&mut input).expect("ERROR READING INPUT...");
        if input == String::from("0\n"){

            for i in data.d.iter(){
                println!("{}: {}", i.pass, i.description);
            }
            
        }else if input == String::from("1\n"){
            println!("Save Password From Input");
            let mut pass = String::from("");
            let mut descript = String::from("");

            println!("Write password: ");

            stdin().read_line(&mut pass).expect("ERROR READING INPUT...");
            
            println!("Write description: ");

            stdin().read_line(&mut descript).expect("ERROR READING INPUT...");
            pass = String::from(pass.strip_suffix('\n').expect("ohhhh"));
            descript = String::from(descript.strip_suffix('\n').expect("ohhhh"));
            data.d.push(Zenzon{pass: pass, description: descript});
        }
        else if input == String::from("3\n"){break;}
        else if input == String::from("2\n") {
            let mut input: String = String::from("");
            println!("Type The Password To Remove (For security): ");
            stdin().read_line(&mut input).expect("ERROR READING INPUT...");

            
            for i in data.clone().d.iter(){
                let temp = input.strip_suffix("\n").expect("Errr");
                if String::from(temp) == i.pass{
                    println!("Removing {}: {}", i.pass, i.description);
                    data.d.swap_remove(data.clone().d.binary_search(i).expect("Error"));
                    fs::write(fname, data.vec_zenzon_to_string()).expect("error writing to file");
                    break;
                }
            }
        }

        save(data, fname, hash);
    }
}



pub fn old_user(path: &str, hash: &mut String, masterpass: &str, data: &mut ZenzonArray){
    let file_bool = read_file(masterpass, false);
    let mut input = String::new();
	let mut file;
    match file_bool.0{
        None => {
			eprintln!("ERROR THE KEY HASH WAS NOT FOUND: IF THE {} DOESNOT EXIST OR IS EMPTY, YOU NEED TO ACCESS THE FILES USING --pass=(recovery_key)\n", masterpass);
			exit(69);
		},
        Some(f) => {
			println!("Intialized login successfully!");
			file = f;
		}
    }

	file.read_to_string(hash).unwrap();


    if file_bool.1 || hash.trim() == ""{
        println!("ERROR THE KEY HASH WAS NOT FOUND: IF THE {} DOESNOT EXIST OR IS EMPTY, YOU NEED TO ACCESS THE FILES USING --pass=(recovery_key)", masterpass);
    }else{
        println!("Enter master password: ");
        stdin().read_line(&mut input).expect("ERROR READING INPUT...");
        data.read_vec_zenzon(&mut Options::File(fs::File::open(path).expect("")));
    }
    if bcrypt::hash_with_salt(input, 6, [69; 16]).expect("Err..").to_string() != *hash{
        println!("Wrong password...");
        exit(69);
    }else{
        println!("Your recovery key is: {}", hash);
        println!("Login successful!");
    }



}



pub fn save(data: &mut ZenzonArray, fname: &str, _hashed: &str){
    fs::write(fname, data.vec_zenzon_to_string()).expect("error writing to file");
}
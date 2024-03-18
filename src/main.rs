mod zenzon;
use zenzon::*;


fn main(){
    // Read Data From Pass File
    let mut data: ZenzonArray = ZenzonArray { d: vec![] };
    let fname = "pass.crypt";
    let masterpass = "hashmeifyoucan.crypt";
    let file = read_file(fname, true);
    let is_created = file.1;
    let mut hash= String::from("");

    if is_created{
        hash = new_user(&masterpass);
    }else{
        old_user(&fname,&mut hash, &masterpass, &mut data);
    }
    
    take_input(&mut data, &fname, &hash);

    // Save Data To File
    save(&mut data, &fname, &hash);
    

}


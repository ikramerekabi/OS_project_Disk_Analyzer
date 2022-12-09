use walkdir::WalkDir;
use std::error::Error;
use std::path::Path;
use filetime::FileTime;
use std::fs;


fn main() ->Result <(), Box<dyn Error>> {

let mut file_and_time_vec = vec![];
  for entry in WalkDir::new("/home/maram/OSProject") {
    
    let entry = entry.unwrap();
    let path = Path::new(entry.path());
    let name = path.file_stem().unwrap().to_os_string().into_string().unwrap();
    let metadata_d = fs::metadata(path).unwrap();
    
    let file_time= FieTime::from_last_modification_time(&metadata_d); 
  // if metadata_d.is_dir()
   //{
   	file_and_time_vec.push((file_time,name));
   //}
}

file_and_time_vec.sort_by(|a,b| b.0.cmp(&a.0));
for item in file_and_time_vec
{
println!("{}\n{} date\n", item.1, item.0);
}


Ok(()) 
}

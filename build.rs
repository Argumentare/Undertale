

fn main()
{
    
    println!("cargo:rustc-link-search=native=lib");
    println!("cargo:rustc-link-lib=static=cplus");
    println!("cargo:rustc-link-lib=raylib"); 
    println!("cargo:rustc-link-lib=m");      
    println!("cargo:rustc-link-lib=GL");      
    println!("cargo:rustc-link-lib=pthread"); 
    println!("cargo:rustc-link-lib=dl");      
    println!("cargo:rustc-link-lib=rt"); 
    
}
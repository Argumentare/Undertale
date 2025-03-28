use std::os::windows::thread;
use fastrand;
use std::iter::repeat_with;

#[derive(Debug)]
pub enum id
{
    eid
    {
        d:String,
    }
}
impl id
{
    pub fn randomid() -> id
    {
        
   
        let newid =  id::eid { d:repeat_with(fastrand::alphanumeric).take(10).collect()};
        return newid
    }
}
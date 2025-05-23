use std::ptr::null_mut;
use std::ffi::CString;

use winapi::um::winuser::{MessageBoxA};

fn main(){
    let title = CString::new("This is the title").unwrap();
    let message = CString::new("blog-0xnullsec.vercel.app\nhttps://t.me/xNullSec\nhttps://github.com/0xNullSec").unwrap();

    unsafe {
        let verify = MessageBoxA(null_mut(), message.as_ptr(), title.as_ptr(), 0x00000000);
        if verify == 1 {
            println!("[+] The user has pressed the button");
        }
    }

}
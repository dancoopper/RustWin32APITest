use windows::Win32::UI::WindowsAndMessaging::MessageBoxW;
use windows::Win32::UI::WindowsAndMessaging::MESSAGEBOX_STYLE;
use windows::core::PCWSTR;
use std::*;
fn main(){
    
    unsafe { //makes a message box that has a caption and a text field
        let s_caption: &str = "Hello";//caption
        let mut v_caption: Vec<u16> = s_caption.encode_utf16().collect();//makes caption into uft16 uni-code
        v_caption.push(0);
        
        let s_text: &str= "this is just a test";
        let mut v_text: Vec<u16> = s_text.encode_utf16().collect();
        v_text.push(0);

        MessageBoxW(
        None, // handle to the owner window of the message box
        PCWSTR(v_text.as_ptr()), //message to be displayed
        PCWSTR(v_caption.as_ptr()), //dialog box title
        MESSAGEBOX_STYLE(0u32)//contents and behavior of the dialog box
        ) 


    };
}
#![windows_subsystem = "windows"]
use windows::Win32::System::Threading::BELOW_NORMAL_PRIORITY_CLASS;
//use windows::Win32::UI::WindowsAndMessaging::MessageBoxW;
//use windows::Win32::UI::WindowsAndMessaging::MESSAGEBOX_STYLE;
use windows::core::PCWSTR;
use windows::core::PWSTR;
use windows::Win32::System::Threading::CreateProcessW;
use std::*;
fn main(){
    
   unsafe{
    /*
    BOOL CreateProcessA(
        [in, optional]      LPCSTR                lpApplicationName,
        [in, out, optional] LPSTR                 lpCommandLine,
        [in, optional]      LPSECURITY_ATTRIBUTES lpProcessAttributes,
        [in, optional]      LPSECURITY_ATTRIBUTES lpThreadAttributes,
        [in]                BOOL                  bInheritHandles,
        [in]                DWORD                 dwCreationFlags,
        [in, optional]      LPVOID                lpEnvironment,
        [in, optional]      LPCSTR                lpCurrentDirectory,
        [in]                LPSTARTUPINFOA        lpStartupInfo,
        [out]               LPPROCESS_INFORMATION lpProcessInformation
      );
      */
      let app_name: &str = "C:\\Windows\\notepad.exe";
      let mut v_app_name: Vec<u16> = app_name.encode_utf16().collect();
      v_app_name.push(0);
      
      let _si: windows::Win32::System::Threading::STARTUPINFOW =  Default::default();
      let mut _pi: *mut windows::Win32::System::Threading::PROCESS_INFORMATION = {std::ptr::null_mut()};
      CreateProcessW(
PCWSTR(v_app_name.as_ptr()), 
      PWSTR(ptr::null_mut()), 
      None, 
      None,
      false,
      BELOW_NORMAL_PRIORITY_CLASS, 
      None, 
      None, 
      &_si, 
      _pi
    );
    
   };
   
   /* 
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
        MESSAGEBOX_STYLE(0u32)| MESSAGEBOX_STYLE(48u32)//contents and behavior of the dialog box
        ) 


    };
    */
}
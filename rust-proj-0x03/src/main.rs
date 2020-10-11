#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

// winapi has since been merged with user32 and kernel32?
// extern crate user32;
// extern crate kernel32;

use std::mem::MaybeUninit;

// ffi is foreign function interface, which is for exchanging c like strings
// os strings are not null terminated, may contain null chars, may be utf8 or utf16
use std::ffi::OsStr;

use std::iter::once;

use std::os::windows::ffi::OsStrExt;

use winapi::shared::windef::
{
  HWND,
  HICON,
  HCURSOR,
  HBRUSH,
  HMENU,
};

use winapi::shared::minwindef::
{
  UINT,
  WPARAM,
  LPARAM,
  LRESULT,
  HINSTANCE,
  DWORD,
  LPVOID,
};

use winapi::um::winuser::
{
  CreateWindowExW,
  DispatchMessageW,
  PeekMessageW,
  RegisterClassW,
  TranslateMessage,
  DefWindowProcW,
};

use winapi::um::winuser::
{
  CS_HREDRAW,
  CS_OWNDC,
  CS_VREDRAW,
  CW_USEDEFAULT,
  PM_REMOVE,
  MSG,
  WNDCLASSW,
  WS_OVERLAPPEDWINDOW,
  WS_VISIBLE,
};

use winapi::um::winnt::
{
  LPCWSTR,
};

// [x] what is unsafe in a function?
//     rtfm and we won't have this question
//     https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html
//     Marking a function as unsafe means that it must be called in an unsafe block
//     It also makes the function scope unsafe
//     Marking WindowProc() as unsafe allows it to call the unsafe winapi::un::winuser::DefWindowProcW
//     function.
//     As it turns out, we don't need to mark WindowProc as unsafe
// [x] why need extern "system" here?
//     https://doc.rust-lang.org/reference/items/external-blocks.html
//     same as extern "C" except on Win32 where it's "stdcall"
extern "system" fn WindowProc( hWnd: HWND,
                                      msg: UINT,
                                      wParam: WPARAM,
                                      lParam: LPARAM )
-> LRESULT
{
  unsafe
  {
    // [ ] DefWindowProcA vs DefWindowProcW?
    //     ...uhh idk lets just use W for everything
    return DefWindowProcW( hWnd, msg, wParam, lParam );
  }
}

// [x] also works? static HWND shWnd = 0;
//     no
static mut shWnd: HWND = 0 as HWND;

fn main()
{
  println!("Hello, world!");

  // [ ] why use unsafe here?
  unsafe
  {
    // [ ] wtf
    //     uhh idk lol
    let windowClassName: Vec<u16> = OsStr::new( "assbutt" ).encode_wide().chain( once( 0 ) ).collect();

    let windowClass = WNDCLASSW
    {
      // [ ] needed?
      //     idk
      style: CS_OWNDC | CS_HREDRAW | CS_VREDRAW,

      // [x] what is Some?
      //     https://docs.rs/winapi/0.3.7/winapi/um/winuser/type.WNDPROC.html
      //     type WNDPROC = Option<unsafe extern "system" fn(_: HWND, _: UINT, _: WPARAM, _: LPARAM) -> LRESULT>;
      //     lpfnWndProc is of type std::option::Option< ... >, which either has Some( ... ) or None()
      //     This is how you can do optional values in rust
      lpfnWndProc: Some( WindowProc ),
      cbClsExtra: 0,
      cbWndExtra: 0,
      hInstance: 0 as HINSTANCE,
      hIcon: 0 as HICON,
      hCursor: 0 as HCURSOR,
      hbrBackground: 0 as HBRUSH,
      lpszMenuName: 0 as LPCWSTR,
      lpszClassName: windowClassName.as_ptr(),
    };

    let errorCode = RegisterClassW( &windowClass );
    // [x] what does assert do in rust?
    //     https://doc.rust-lang.org/std/macro.assert.html
    //     invokes panic! if untrue
    assert!( errorCode != 0, "window class registration failed" );

    shWnd = CreateWindowExW( 0 as DWORD,
                             windowClassName.as_ptr(),
                             0 as LPCWSTR,
                             WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                             CW_USEDEFAULT,
                             CW_USEDEFAULT,
                             400,
                             400,
                             0 as HWND,
                             0 as HMENU,
                             0 as HINSTANCE,
                             0 as LPVOID );

     assert!( shWnd != ( 0 as HWND ), "failed to open window" );
  }

   loop
   {
     unsafe
     {
       let mut msg = MaybeUninit::< MSG >::uninit();
       while PeekMessageW( msg.as_mut_ptr(), shWnd, 0, 0, PM_REMOVE ) > 0
       {
         TranslateMessage( msg.as_ptr() );
         DispatchMessageW( msg.as_ptr() );
       }
     }
   }
}


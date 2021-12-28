use crate::{user32::HDC, LPVOID};

pub type HGDIOBJ = LPVOID;


#[link(name = "gdi32", kind = "dylib")]
extern "system" {

    /// https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createcompatiblebitmap
    /// ```
    /// HBITMAP CreateCompatibleBitmap(
    ///   [in] HDC hdc,
    ///   [in] int cx,
    ///   [in] int cy
    /// );
    /// ```
    pub fn CreateCompatibleBitmap(hdc: HDC, cx: i32, cy: i32);

    /// https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createcompatibledc
    /// HDC CreateCompatibleDC(
    ///   [in] HDC hdc
    /// );
    pub fn CreateCompatibleDC(hdc: HDC);

    /// https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectobject
    /// HGDIOBJ SelectObject(
    ///   [in] HDC     hdc,
    ///   [in] HGDIOBJ h
    /// );
    pub fn SelectObject(hdc: HDC, h: HGDIOBJ) -> HGDIOBJ;
}

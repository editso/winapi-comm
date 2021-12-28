#![allow(unused)]
#![allow(non_camel_case_types)]

use c2rs::c2rs_def;

use crate::{BOOL, DWORD, LPCSTR, LPVOID};

pub type HDESK = LPVOID;
pub type ACCESS_MASK = DWORD;
pub type LPSECURITY_ATTRIBUTES = *const SECURITY_ATTRIBUTES;
pub type HDC = LPVOID;
pub type HWND = LPVOID;

c2rs_def!(

    struct SECURITY_ATTRIBUTES {
        DWORD nLength;
        LPVOID lpSecurityDescriptor;
        BOOL bInheritHandle;
    };

);

#[link(name = "user32", kind = "dylib")]
extern "system" {
    /// https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createdesktopa
    /// ```
    /// HDESK CreateDesktopA(
    ///   [in]           LPCSTR                lpszDesktop,
    ///   LPCSTR                lpszDevice,
    ///   DEVMODEA              *pDevmode,
    /// [in]           DWORD                 dwFlags,
    /// [in]           ACCESS_MASK           dwDesiredAccess,
    /// [in, optional] LPSECURITY_ATTRIBUTES lpsa
    /// );
    /// ```
    pub fn CreateDesktopA(
        lpszDesktop: LPCSTR,
        lpszDevice: LPCSTR,
        pDevmode: LPVOID,
        dwFlags: DWORD,
        dwDesiredAccess: ACCESS_MASK,
        lpsa: LPSECURITY_ATTRIBUTES,
    ) -> HDESK;

    ///
    /// https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-closedesktop
    /// ```
    /// BOOL CloseDesktop(
    ///   [in] HDESK hDesktop
    /// );
    /// ```
    pub fn CloseDesktop(hDesktop: HDESK) -> BOOL;

    /// https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-opendesktopa
    /// ```
    /// HDESK OpenDesktopA(
    ///   [in] LPCSTR      lpszDesktop,
    ///   [in] DWORD       dwFlags,
    ///   [in] BOOL        fInherit,
    ///   [in] ACCESS_MASK dwDesiredAccess
    ///
    /// )
    /// ```;
    pub fn OpenDesktopA(
        lpszDesktop: LPCSTR,
        dwFlags: DWORD,
        fInherit: BOOL,
        dwDesiredAccess: ACCESS_MASK,
    ) -> HDESK;

    /// https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setthreaddesktop
    /// ```
    /// BOOL SetThreadDesktop(
    ///   [in] HDESK hDesktop
    /// );
    /// ```
    pub fn SetThreadDesktop(hDesktop: HDESK) -> BOOL;

    /// https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getthreaddesktop
    /// ```
    /// HDESK GetThreadDesktop(
    ///   [in] DWORD dwThreadId
    /// );
    /// ```
    pub fn GetThreadDesktop(dwThreadId: DWORD) -> HDESK;

    /// https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdesktopwindow
    /// ```
    /// HWND GetDesktopWindow();
    /// ```
    pub fn GetDesktopWindow() -> HDESK;

    /// https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdc
    /// ```
    /// HDC GetDC(
    ///   [in] HWND hWnd
    /// );
    /// ```
    pub fn GetDC(hwnd: HWND) -> HDC;
}

#[cfg(test)]
mod tests {
    use super::GetDesktopWindow;

    #[test]
    fn test_desktop() {
        unsafe {
            let hdesk = GetDesktopWindow();
            
            assert!(!hdesk.is_null())
        }
    }
}

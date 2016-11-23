#![cfg(windows)]

use std::cmp::Ordering;
use std::ffi::{CStr, CString};
use std::mem::{zeroed, size_of};
use winapi::minwindef::{DWORD, FALSE, TRUE};
use winapi::winnt::{HANDLE, PROCESS_ALL_ACCESS};
use winapi::tlhelp32::{PROCESSENTRY32, TH32CS_SNAPPROCESS};
use kernel32::{CreateToolhelp32Snapshot, Process32First, Process32Next, OpenProcess};

/// The `WindowsProcess` struct.
#[derive(Clone)]
pub struct WindowsProcess {
    /// The process handle.
    handle: HANDLE,
}

/// Implements `WindowsProcess`.
impl WindowsProcess {
    /// Constructs a new `WindowsProcess`.
    fn new(handle: HANDLE) -> WindowsProcess {
        WindowsProcess { handle: handle }
    }
    /// Finds the first process with the specified name.
    pub fn find_by_name(name: &str) -> Option<WindowsProcess> {
        let processes = WindowsProcess::find_all_by_name(name);
        match processes.len() {
            0 => None,
            _ => Some(processes[0].clone()),
        }
    }
    /// Finds all processes with the specified name.
    pub fn find_all_by_name(name: &str) -> Vec<WindowsProcess> {
        let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
        let dest_path = CString::new(name).unwrap();
        let mut vec = Vec::<WindowsProcess>::new();
        let mut entry = unsafe { zeroed::<PROCESSENTRY32>() };
        entry.dwSize = size_of::<PROCESSENTRY32>() as DWORD;
        let loop_func = |entry: PROCESSENTRY32, vec: &mut Vec<WindowsProcess>| {
            let path = unsafe { CString::from(CStr::from_ptr(entry.szExeFile.as_ptr())) };
            if path.cmp(&dest_path) == Ordering::Equal {
                let handle = unsafe { OpenProcess(PROCESS_ALL_ACCESS, FALSE, entry.th32ProcessID) };
                vec.push(WindowsProcess::new(handle));
            }
        };
        if unsafe { Process32First(snapshot, &mut entry) == TRUE } {
            while {
                loop_func(entry.clone(), &mut vec);
                unsafe { Process32Next(snapshot, &mut entry) == TRUE }
            } {}
        }
        vec
    }
}
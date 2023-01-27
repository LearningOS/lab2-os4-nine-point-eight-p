//! Process management syscalls

use core::iter::zip;

use crate::config::MAX_SYSCALL_NUM;
use crate::mm::translated_byte_buffer;
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, current_task_info, current_user_token};
use crate::timer::get_time_us;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

#[derive(Clone, Copy)]
pub struct TaskInfo {
    pub status: TaskStatus,
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    pub time: usize,
}

pub fn sys_exit(exit_code: i32) -> ! {
    info!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

// YOUR JOB: 引入虚地址后重写 sys_get_time
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    // Get time
    let us = get_time_us();
    let time = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };
    // Convert time object into byte slice
    let len = core::mem::size_of::<TimeVal>();
    let time = unsafe {
        core::slice::from_raw_parts((&time as *const _) as *const u8, len)
    };
    // Translate virtual address into physical addresses
    let buffers = translated_byte_buffer(current_user_token(), _ts as *const u8, _tz);
    // Copy to buffers
    let mut pos = 0;
    for buffer in buffers {
        let copy_len = buffer.len().min(len - pos);
        buffer.copy_from_slice(&time[pos..(pos + copy_len + 1)]);
        pos += copy_len;
    }
    0
}

// CLUE: 从 ch4 开始不再对调度算法进行测试~
pub fn sys_set_priority(_prio: isize) -> isize {
    -1
}

// YOUR JOB: 扩展内核以实现 sys_mmap 和 sys_munmap
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    -1
}

pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    -1
}

// YOUR JOB: 引入虚地址后重写 sys_task_info
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    let task_info = current_task_info();
    copy_to_raw(ti, None, &task_info);
    0
}

fn copy_to_raw<T>(ptr: *mut T, size: Option<usize>, object: &T) {
    // Convert object into byte slice
    let object_len = core::mem::size_of::<T>();
    let object_slice = unsafe {
        core::slice::from_raw_parts((object as *const _) as *const u8, object_len)
    };
    // Translate virtual address into physical addresses
    let size = size.unwrap_or(object_len);
    let buffers = translated_byte_buffer(current_user_token(), ptr as *const u8, size);
    // Copy to buffers
    let mut pos = 0;
    for buffer in buffers {
        let copy_len = buffer.len().min(object_len - pos);
        buffer.copy_from_slice(&object_slice[pos..(pos + copy_len + 1)]);
        pos += copy_len;
    }
}
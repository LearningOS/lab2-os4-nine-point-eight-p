//! Process management syscalls

use core::iter::Map;

use riscv::addr::BitField;

use crate::config::{MAX_SYSCALL_NUM, PAGE_SIZE};
use crate::mm::{VirtPageNum, VirtAddr, MapPermission};
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, current_user_token, map_for_current};
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
    let _us = get_time_us();
    // unsafe {
    //     *ts = TimeVal {
    //         sec: us / 1_000_000,
    //         usec: us % 1_000_000,
    //     };
    // }
    0
}

// CLUE: 从 ch4 开始不再对调度算法进行测试~
pub fn sys_set_priority(_prio: isize) -> isize {
    -1
}

// YOUR JOB: 扩展内核以实现 sys_mmap 和 sys_munmap
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    // Get virtual address
    let start: VirtAddr = _start.into();
    let end: VirtAddr = (_start + _len).into(); // not aligned
    // Check start
    if start.aligned() == false {
        return -1;
    }
    // Check port
    if _port.get_bits(3..) != 0 || _port.get_bits(0..3) == 0 {
        return -1;
    }
    let perm = MapPermission::U | MapPermission::from_bits((_port.get_bits(0..3) << 1) as u8).unwrap();
    // Check length
    if _len == 0 {
        return 0;
    }
    // Map
    map_for_current(start, end, perm);
    0
}

pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    -1
}

// YOUR JOB: 引入虚地址后重写 sys_task_info
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    -1
}

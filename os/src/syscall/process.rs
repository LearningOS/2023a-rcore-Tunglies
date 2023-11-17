//! Process management syscalls
use core::mem::size_of;

use crate::{
    config::MAX_SYSCALL_NUM,
    task::{
        change_program_brk, exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, memory_map, is_memory_mapped, memory_unmap, get_current_token, get_current_task_status, get_currnt_task_syscall_times,
        // get_currnt_task_syscall_times,
    }, mm::{VirtAddr, MapPermission, translated_byte_buffer}, timer::{get_time_us, get_time_ms}
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let token = get_current_token();
    let ptr = _ts as *const u8;
    let len = size_of::<TimeVal>();
    let buffers = translated_byte_buffer(token, ptr, len);
    
    let us = get_time_us();
    let time_val = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };

    let mut time_val_ptr = &time_val as *const _ as *const u8;
    
    for buffer in buffers {
        unsafe {
            time_val_ptr.copy_to(buffer.as_mut_ptr(), buffer.len());
            time_val_ptr = time_val_ptr.add(buffer.len());
        }
    }

    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    debug!("entrying fn::sys_task_info");
    trace!("kernel: sys_task_info NOT IMPLEMENTED YET!");
    let token = get_current_token();
    let ptr = _ti as *const u8;
    let len = size_of::<TaskInfo>();
    let buffers = translated_byte_buffer(token, ptr, len);
    
    let ti = TaskInfo {
        status: get_current_task_status(),
        syscall_times: get_currnt_task_syscall_times(),
        time: get_time_ms(),
    };

    let mut ti_ptr = &ti as *const _ as *const u8;

    for buffer in buffers {
        unsafe {
            ti_ptr.copy_to(buffer.as_mut_ptr(), buffer.len());
            ti_ptr = ti_ptr.add(buffer.len());
        }
    }

    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    debug!("entrying fn::sys_mmap");
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    let start_va = VirtAddr(_start);
    let end_va = VirtAddr(_start + _len);

    debug!("check virtual address ok");
    if !start_va.aligned() {
        return -1;
    }
    
    debug!("recived _port: {:?}", _port);
    if _port & !0x7 != 0 || _port & 0x7 == 0 {
        return -1;
    }
    
    debug!("initiall permission");
    // Let's go back previous commit permission setting.
    // I still love my code anyway.
    let mut permission = MapPermission::U;
    if _port & (1<<0) != 0 {
        permission |= MapPermission::R;
    }
    if _port & (1<<1) != 0 {
        permission |= MapPermission::W;
    }
    if _port & (1<<3) != 0 {
        permission |= MapPermission::X;
    }
    debug!("permission: {:?}", permission);

    debug!("check memory is mapped");
    if !is_memory_mapped(start_va, end_va, false) {
        return -1;
    }

    debug!("memory not mapped, mapping");
    memory_map(start_va, end_va, permission);

    debug!("exiting fn::sys_mmap\n");
    0
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    debug!("entrying fn::sys_munmap");
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    let start_va = VirtAddr(_start);
    let end_va = VirtAddr(_start+_len);

    debug!("check virtual address ok");
    if !start_va.aligned() {
        return -1;
    }

    debug!("check memory is mapped");
    if !is_memory_mapped(start_va, end_va, true) {
        return -1;
    }

    debug!("memory mapped, unmapping");
    memory_unmap(start_va, end_va);

    debug!("exiting fn::sys_munmap\n");
    0
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}

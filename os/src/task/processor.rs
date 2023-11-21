//!Implementation of [`Processor`] and Intersection of control flow
//!
//! Here, the continuous operation of user apps in CPU is maintained,
//! the current running state of CPU is recorded,
//! and the replacement and transfer of control flow of different applications are executed.

use super::{__switch};
use super::{fetch_task, TaskStatus};
use super::{TaskContext, TaskControlBlock};
use crate::config::MAX_SYSCALL_NUM;
use crate::mm::{VirtAddr, MapPermission};
use crate::sync::UPSafeCell;
use crate::trap::TrapContext;
use alloc::sync::Arc;
use lazy_static::*;

/// Processor management structure
pub struct Processor {
    ///The task currently executing on the current processor
    current: Option<Arc<TaskControlBlock>>,

    ///The basic control flow of each core, helping to select and switch process
    idle_task_cx: TaskContext,
}

impl Processor {
    ///Create an empty Processor
    pub fn new() -> Self {
        Self {
            current: None,
            idle_task_cx: TaskContext::zero_init(),
        }
    }

    ///Get mutable reference to `idle_task_cx`
    fn get_idle_task_cx_ptr(&mut self) -> *mut TaskContext {
        &mut self.idle_task_cx as *mut _
    }

    ///Get current task in moving semanteme
    pub fn take_current(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.current.take()
    }

    ///Get current task in cloning semanteme
    pub fn current(&self) -> Option<Arc<TaskControlBlock>> {
        self.current.as_ref().map(Arc::clone)
    }

    /// Is Maped
    pub fn is_mapped(&self, start_va: VirtAddr, end_va: VirtAddr, mapped: bool) -> bool {
        self.current().unwrap().is_mapped(start_va, end_va, mapped)
    }

    /// MMAP
    pub fn mmap(&self, start_va: VirtAddr, end_va: VirtAddr, permission: MapPermission) {
        self.current().unwrap().mmap(start_va, end_va, permission);
    }

    /// Unmap
    pub fn unmap(&self, start_va: VirtAddr, end_va: VirtAddr) {
        self.current().unwrap().unmap(start_va, end_va);
    }

    /// Status
    pub fn status(&self) -> TaskStatus {
        self.current().unwrap().status()
    }

    /// Increase syscall
    pub fn increase_syscall(&self, syscall_id: usize) {
        self.current().unwrap().increase_syscall(syscall_id);
    }

    /// Current syscall
    pub fn current_syscall(&self) -> [u32; MAX_SYSCALL_NUM] {
        self.current().unwrap().current_syscall()
    }

    /// spwan a process
    pub fn spwan(&self, data: &[u8]) -> isize {
        let current = self.current().unwrap();
        self.current().unwrap().spwan(current, data)
    }
}

lazy_static! {
    pub static ref PROCESSOR: UPSafeCell<Processor> = unsafe { UPSafeCell::new(Processor::new()) };
}

///The main part of process execution and scheduling
///Loop `fetch_task` to get the process that needs to run, and switch the process through `__switch`
pub fn run_tasks() {
    loop {
        let mut processor = PROCESSOR.exclusive_access();
        if let Some(task) = fetch_task() {
            let idle_task_cx_ptr = processor.get_idle_task_cx_ptr();
            // access coming task TCB exclusively
            let mut task_inner = task.inner_exclusive_access();
            let next_task_cx_ptr = &task_inner.task_cx as *const TaskContext;
            task_inner.task_status = TaskStatus::Running;
            // release coming task_inner manually
            drop(task_inner);
            // release coming task TCB manually
            processor.current = Some(task);
            // release processor manually
            drop(processor);
            unsafe {
                __switch(idle_task_cx_ptr, next_task_cx_ptr);
            }
        } else {
            warn!("no tasks available in run_tasks");
        }
    }
}

/// Get current task through take, leaving a None in its place
pub fn take_current_task() -> Option<Arc<TaskControlBlock>> {
    PROCESSOR.exclusive_access().take_current()
}

/// Get a copy of the current task
pub fn current_task() -> Option<Arc<TaskControlBlock>> {
    PROCESSOR.exclusive_access().current()
}


/// Spawn task
pub fn current_task_spwan(data: &[u8]) -> isize {
    PROCESSOR.exclusive_access().spwan(data)
}

/// Is Mapped
pub fn current_is_mapped(start_va: VirtAddr, end_va: VirtAddr, mapped: bool) -> bool {
    PROCESSOR.exclusive_access().is_mapped(start_va, end_va, mapped)
}

/// MMAP
pub fn current_mmap(start_va: VirtAddr, end_va: VirtAddr, permission: MapPermission) {
    PROCESSOR.exclusive_access().mmap(start_va, end_va, permission);
}

/// UnMMAP
pub fn current_unmap(start_va: VirtAddr, end_va: VirtAddr) {
    PROCESSOR.exclusive_access().unmap(start_va, end_va);
}

/// Get the current user token(addr of page table)
pub fn current_user_token() -> usize {
    let task = current_task().unwrap();
    task.get_user_token()
}

/// Get the current task status
pub fn current_task_status() -> TaskStatus {
    PROCESSOR.exclusive_access().status()
}

/// Increase the current task syscall times
pub fn current_task_increase_syscall(syscall_id: usize) {
    PROCESSOR.exclusive_access().increase_syscall(syscall_id);
}

/// Get the current task syscall times
pub fn current_task_syscall() -> [u32; MAX_SYSCALL_NUM] {
    PROCESSOR.exclusive_access().current_syscall()
}

///Get the mutable reference to trap context of current task
pub fn current_trap_cx() -> &'static mut TrapContext {
    current_task()
        .unwrap()
        .inner_exclusive_access()
        .get_trap_cx()
}

///Return to idle control flow for new scheduling
pub fn schedule(switched_task_cx_ptr: *mut TaskContext) {
    let mut processor = PROCESSOR.exclusive_access();
    let idle_task_cx_ptr = processor.get_idle_task_cx_ptr();
    drop(processor);
    unsafe {
        __switch(switched_task_cx_ptr, idle_task_cx_ptr);
    }
}

#![no_std]
#![no_main]

mod init;
mod page;
mod uart;

use core::fmt::Write;
use uart::UartWriter;

use crate::page::PageTable;

/// The main kernel function.
///
/// It makes the following assumptions:
/// 1) hart_id is set to current hart executing the code
/// 2) dev_tree contains physical address of device tree
/// 3) Sv39 paging is enabled and page_table contains physical address of
///     currently used page table
/// 4) The page table identity maps dev_tree and page_table
pub fn main(hart_id: usize, dev_tree: *const u8, page_table: *mut PageTable) {
    uprintln!("Kernel initialized in virtual memory.");
    let pc: usize;
    let kernel_start: usize;
    let kernel_end: usize;
    unsafe {
        core::arch::asm!(
            "auipc {pc}, 0",
            "la {kernel_start}, __kernel_start",
            "la {kernel_end}, __kernel_end",
            pc = out(reg) pc,
            kernel_start = out(reg) kernel_start,
            kernel_end = out(reg) kernel_end
        )
    }
    uprintln!("Current PC: 0x{:x}", pc);
    uprintln!("Dev-tree at: 0x{:x}", dev_tree as usize);

    uprintln!("Kernel start at: 0x{:x}", kernel_start);
    uprintln!("Kernel end at: 0x{:x}", kernel_end);

    uprintln!("Page table at: 0x{:x}", page_table as usize);

    // The job here is to create a new, corrected page table
    // To do so, first we need to discover available memory
    // Then create the mapping and update satp
}

use crate::page::{Entry, Flag::*, PageTable};
use crate::uart::UartWriter;
use crate::{main, uprintln};
use core::fmt::Write;

static mut PT: PageTable = unsafe { core::mem::transmute([0usize; 512]) };

/// The first function called by the bootloader.
///
/// It is only responsible for setting up temporary stack
/// and then jumping to temporary virtual memory initialization.
#[unsafe(link_section = ".text.start")]
#[unsafe(no_mangle)]
pub unsafe fn _start(hart_id: usize, dev_tree: *const u8) -> ! {
    unsafe {
        core::arch::asm!(
            "la sp, __stack_top",
            "call {vmem_init}",
            vmem_init = sym vmem_init,
            options(noreturn),
            in("a0") hart_id,
            in("a1") dev_tree,
        )
    }
}

const KERNEL_VIRT_BASE: usize = 0xffffffff80200000;
const KERNEL_PHYS_BASE: usize = 0x80200000;

/// Temporary virtual memory initialization.
///
/// Now the program is in very sensitive position - it is linked to run
/// at higher-half, virtual addresses, but currently is loaded into physical
/// memory. This function creates a fast, heuristic mapping of itself to
/// virtual memory: one identity mapping, and one higher-half mapping.
/// For UART devices, also the lowest 0x1000_0000 addresses are identity
/// mapped. It uses three 1-GiB pages and disregards actual memory bounds.
///
/// After setting up satp register it jumps to `kernel_init` in high memory.
#[unsafe(link_section = ".text.vmem_init")]
#[unsafe(no_mangle)]
pub unsafe fn vmem_init(hart_id: usize, dev_tree: *const u8) -> ! {
    unsafe {
        /*
         * PT.map(KERNEL_VIRT_BASE, KERNEL_PHYS_PAGE, PageSize::L2, &[Valid, Read, Write, Execute]);
         * PT.map(KERNEL_PHYS_BASE, KERNEL_PHYS_PAGE, PageSize::L2, &[Valid, Read, Write, Execute]);
         * PT.map(0, 0, PageSize::L2);
         */
        let kernel_page = Entry::new(((KERNEL_PHYS_BASE >> 30) & 0x3ff_ffff) << 28)
            .flags([Valid, Read, Write, Execute].into_iter());

        // higher-half mapping
        let kernel_high = (KERNEL_VIRT_BASE >> 30) & 0x1FF;
        PT.entries[kernel_high] = kernel_page;

        // identity mapping
        let kernel_id = (KERNEL_PHYS_BASE >> 30) & 0x1FF;
        PT.entries[kernel_id] = kernel_page;

        // uart map
        PT.entries[0] = Entry::new(0).flags([Valid, Read, Write].into_iter());

        let root_ppn = (&raw const PT.entries as usize) >> 12;
        let satp_val = (8 << 60) | root_ppn;

        core::arch::asm!(
            "csrw satp, {0}",
            "sfence.vma", // force CPU to flush TLB
            in(reg) satp_val
        );

        let addr: usize = 0xffffffff80204000; // kernel_init address in higher half
        let kernel_init_high: fn(hart_id: usize, dev_tree: *const u8, pt: *mut PageTable) -> ! =
            core::mem::transmute(addr);

        kernel_init_high(hart_id, dev_tree, &raw mut PT)
    }
}

/// First function called after virtual memory is initialized.
///
/// It's only job is to update stack address and call kernel main function.
/// After main returns, loop infinitely.
#[unsafe(link_section = ".text.kernel_init")]
#[unsafe(no_mangle)]
pub unsafe fn kernel_init(hart_id: usize, dev_tree: *const u8, page_table: *mut PageTable) -> ! {
    unsafe {
        core::arch::asm!(
            "la sp, __stack_top",
            "call {main}",
            "call {wfi_loop}",
            main = sym main,
            wfi_loop = sym wfi_loop,
            options(noreturn),
            in("a0") hart_id,
            in("a1") dev_tree,
            in("a2") page_table,
        )
    }
}

pub fn wfi_loop() -> ! {
    uprintln!("Falling into infinite loop...");
    loop {
        unsafe { core::arch::asm!("wfi") }
    }
}

#[panic_handler]
fn panic(p: &core::panic::PanicInfo) -> ! {
    uprintln!("Kernel panicked! Reason: {}", p);
    wfi_loop();
}

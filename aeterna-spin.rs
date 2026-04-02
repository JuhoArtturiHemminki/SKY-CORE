#![no_std]
use std::arch::asm;
use std::sync::atomic::{compiler_fence, Ordering};

pub const AETERNA_BASE: *mut u32 = 0x4000_1000 as *mut u32;
pub const COHERENCE_LOCK_SIG: u32 = 0xFFFFFFFF;
pub const PHASE_ZERO: u32 = 0x0000_0000;
pub const PHASE_PI: u32 = 0x8000_0000;

#[repr(align(64))]
pub struct AeternaCoherenceFabric {
    monitor_reg: *mut u32,
}

impl AeternaCoherenceFabric {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            monitor_reg: AETERNA_BASE,
        }
    }

    #[inline(always)]
    pub unsafe fn establish_phase_lock(&self) {
        asm!(
            "str {val}, [{addr}]",
            "dmb sy",
            "isb",
            val = in(reg) COHERENCE_LOCK_SIG,
            addr = in(reg) self.monitor_reg,
            options(nostack, preserves_flags)
        );
        compiler_fence(Ordering::SeqCst);
    }

    #[inline(always)]
    pub unsafe fn check_coherence_stability(&self) -> bool {
        let status: u32;
        asm!(
            "ldr {val}, [{addr}]",
            "dmb sy",
            val = out(reg) status,
            addr = in(reg) self.monitor_reg,
            options(nostack, preserves_flags)
        );
        status == COHERENCE_LOCK_SIG
    }

    #[inline(always)]
    pub unsafe fn sync_barrier(&self) {
        asm!(
            "dmb sy",
            "isb",
            options(nostack, preserves_flags)
        );
        compiler_fence(Ordering::SeqCst);
    }

    #[inline(always)]
    pub unsafe fn resolve_phase_state(&self, gate_addr: *mut u32) -> bool {
        let val: u32;
        asm!(
            "dmb sy",
            "ldr {val}, [{addr}]",
            val = out(reg) val,
            addr = in(reg) gate_addr,
            options(nostack, preserves_flags)
        );
        val < 0x4000_0000
    }
}

pub mod low_level {
    use super::*;

    #[inline(always)]
    pub unsafe fn atomic_phase_inject(addr: *mut u32, phase: u32) {
        asm!(
            "str {val}, [{addr}]",
            "dmb sy",
            val = in(reg) phase,
            addr = in(reg) addr,
            options(nostack, preserves_flags)
        );
        compiler_fence(Ordering::SeqCst);
    }
}

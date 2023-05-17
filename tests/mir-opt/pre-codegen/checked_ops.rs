// compile-flags: -O -Zmir-opt-level=2 -Cdebuginfo=2
// needs-unwind
// ignore-debug

#![crate_type = "lib"]
#![feature(step_trait)]

// EMIT_MIR checked_ops.step_forward.PreCodegen.after.mir
pub fn step_forward(x: u32, n: usize) -> u32 {
    std::iter::Step::forward(x, n)
}

// EMIT_MIR checked_ops.checked_shl.PreCodegen.after.mir
pub fn checked_shl(x: u32, rhs: u32) -> Option<u32> {
    x.checked_shl(rhs)
}

// EMIT_MIR checked_ops.ilog2.PreCodegen.after.mir
pub fn ilog2(x: u32) -> u32 {
    x.ilog2()
}

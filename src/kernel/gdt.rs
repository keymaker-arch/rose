// use lazy_static::lazy_static;
// use x86_64::{
//     VirtAddr,
//     structures::{
//         tss::TaskStateSegment,
//         gdt::{GlobalDescriptorTable, Descriptor},
//     },

// };

// pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

// lazy_static! {
//     static ref TSS: TaskStateSegment = {
//         let mut tss = TaskStateSegment::new();
//         tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
//             const STACK_SIZE: usize = 4096 * 5;
//             static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

//             let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
//             let stack_end = stack_start + STACK_SIZE.try_into().unwrap();
//             stack_end
//         };
//         tss
//     };
// }

// lazy_static! {
//     static ref GDT: GlobalDescriptorTable = {
//         let mut gdt = GlobalDescriptorTable::new();
//         gdt.append(Descriptor::kernel_code_segment());
//         gdt.append(Descriptor::tss_segment(&TSS));
//         gdt
//     };
// }

// pub fn gdt_init() {
//     GDT.load();
// }
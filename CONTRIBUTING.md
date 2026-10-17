# Contributing to bigL4

Contributions are very welcome! Use this document while contributing to the kernel to guide yourself.

## Bad Ideas & Bad Code

Say you have an idea, and/or you have a general idea of how to work on the kernel. You are
encouraged to contribute! Don't be ashamed to write messy, inconsistent code. As long as your idea
makes sense, the code for it can be polished to fit into the project.

For others that are more experienced developers, make sure that you give **constructive** criticism
of code that you deem to be "bad." Writing a kernel is hard, and contributing is a great way for
people to learn better practices.

# Formatting

To format Rust code, use `rustfmt` (or `cargo fmt`), which applies the project's formatting rules
automatically.

All lines, including comments, should have a line length limit of 100 characters (unless a very
specific and unavoidable expression is used that cannot be broken into multiple lines, such as a
large markdown table.)

## Shorthands

Some terms in source code are shortened to be more compact. Below is a list of those terms
in alphebetical order.

> [!NOTE]
> This is probably not comprehensive.

| Shorthand | Denotation                                           |
| --------- | ---------------------------------------------------- |
| `addr`    | memory address                                       |
| `arch`    | architecture                                         |
| `bss`     | block started by symbol (uninitialized data segment) |
| `asm`     | assembly                                             |
| `config`  | configuration                                        |
| `dev`     | device                                               |
| `fdt`     | flattened device tree                                |
| `hart`    | hardware thread                                      |
| `init`    | initialize                                           |
| `mem`     | memory                                               |
| `mm`      | memory management                                    |
| `phys`    | physical                                             |
| `ptr`     | memory pointer                                       |
| `reg`     | register                                             |
| `sbi`     | supervisor binary interface                          |
| `util`    | utility                                              |
| `vec`     | vector                                               |
| `virt`    | virtual                                              |

For example, the path `crate::arch::mm::virt_to_phys_ptr` would expand to
`crate::architecture::memory_management::virtual_to_physical_memory_pointer`.

Here is what code would look like if shorthands weren't used:

```rust
use core::arch::naked_asm;
use core::mem::transmute;

use riscv::register::scause::Scause;

use crate::architecture::riscv64::supervisor_binary_interface::debug_console::{dbcn, dbcnln};
use crate::cpu::CpuContext;
use crate::utility::ansi;

#[unsafe(link_section = ".text.trap")]
#[unsafe(naked)]
pub(crate)unsafe extern "C" fn trap_vector() -> ! {
    naked_asm!(...)
}

#[unsafe(link_section = ".text.trap")]
#[unsafe(no_mangle)]
unsafe extern "C" fn kernel_trap_handler(context: &CpuContext) -> ! {
    if unsafe { transmute::<u64, Scause>(context.trap_frame.scause) }.is_interrupt() {
        dbcnln!("TRAPPED (interrupt) on hart {}", context.cpu_identifier);
    } else {
        dbcn!("{}", ansi::RED);

        dbcnln!("TRAPPED (exception) on hart {}", context.cpu_identifier);

        // GPRs
        dbcnln!("general purpose registers:");
        for index in 0..=31 {
            dbcnln!("  x{:<7}{:#018x}", index, context.trap_frame.x[i]);
        }
        // FPRs
        dbcnln!("floating point registers:");
        for index in 0..=31 {
            dbcnln!("  f{:<7}{}", index, context.trap_frame.f[i]);
        }
        // CSRs
        dbcnln!("control and status registers:");
        dbcnln!("  fcsr    {:#018x}", context.trap_frame.fcsr);
        dbcnln!("  sepc    {:#018x}", context.trap_frame.sepc);
        dbcnln!("  sstatus {:#018x}", context.trap_frame.sstatus);
        dbcnln!("  scause  {:#018x}", context.trap_frame.scause);
        dbcnln!("  stval   {:#018x}", context.trap_frame.stval);
        dbcnln!("  satp    {:#018x}", context.trap_frame.satp);

        dbcn!("{}", ansi::RESET);
    }

    panic!("trap handler attempted to return");
}
```

Not only is it inconsistent with the Rust standard library, but it is also incredibly verbose and
hard to read. This would be much less of an issue with a language like Swift, which is designed for
concise-yet-verbose code.

It's okay to forget shorthands; Shorthands can be applied later on in patches.

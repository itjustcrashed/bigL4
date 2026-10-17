# System Calls

This document outlines every single system call available to user space programs running on the Neon
kernel.

## Layout

#### RISC-V

On RISC-V architectures, performed using the `ecall` instruction. The calls used on these
architectures are inspired by the calls specified by the
[Supervisor Binary Interface](https://docs.riscv.org/reference/sbi/index.html) standard.

- **→** indicates the value of the register BEFORE the call
- **←** indicates the value of the register AFTER the call

| Register | Description  | ABI name |
| -------- | ------------ | -------- |
| → `a0`   | argument 0   | `arg0`   |
| → `a1`   | argument 1   | `arg1`   |
| → `a2`   | argument 2   | `arg2`   |
| → `a3`   | argument 3   | `arg3`   |
| → `a4`   | argument 4   | `arg4`   |
| → `a5`   | argument 5   | `arg5`   |
| → `a6`   | argument 6   | `arg6`   |
| → `a7`   | call code    | `callc`  |
| ← `a0`   | error code   | `errorc` |
| ← `a1`   | return value | `retv`   |

## Returning

Each call returns the following structure, passed through the `errorc` and `retv` registers:

```c
struct nn_ret {
  signed long error; // errorc
  union { // retv
    signed long ivalue;
    unsigned long uvalue;
  };
};
```

| Error | C Name        | Description              |
| ----- | ------------- | ------------------------ |
| `0`   | `NN_ESUCCESS` | success                  |
| `-1`  | `NN_EBADCODE` | bad code                 |
| `-2`  | `NN_EBAD_ARG` | bad argument             |
| `-3`  | `NN_ENOFMEM`  | no free memory available |

## List of Calls

> [!IMPORTANT]
> Each system call has a C function prototype available. The functions are implemented in `libneon`

### `nn_sleep_thread`

Pauses execution of the current thread for the provided duration in milliseconds.

- Returns the duration that execution was paused in milliseconds
- Fails with `NN_EBAD_ARG` if `duration` is zero

```c
[[gnu::naked]] struct nn_ret sleep_thread(unsigned long duration);
```

//! bare0.rs
//!
//! Simple bare metal application
//! What it covers:
//! - constants
//! - global (static) variables
//! - checked vs. wrapping arithmetics
//! - safe and unsafe code
//! - making a safe API
//! - assertions
//! - panic handling

// build without the Rust standard library
#![no_std]
// no standard main, we declare main using [entry]
#![no_main]

// Minimal runtime / startup for Cortex-M microcontrollers
//extern crate cortex_m_rt as rt;
// Panic handler, for textual output using semihosting
extern crate panic_semihosting;
// Panic handler, infinite loop on panic
// extern crate panic_halt;

// import entry point
use cortex_m_rt::entry;

// a constant (cannot be changed at run-time)
const X_INIT: u32 = 10;
//const X_INIT: u32 = core::u32::MAX;

// global mutable variables (changed using unsafe code)
static mut X: u32 = X_INIT;
static mut Y: u32 = 0;

#[entry]
fn main() -> ! {
    // local mutable variable (changed in safe code)
    let mut x = unsafe { X };

    loop {
     // x += 1; // <- place breakpoint here (3)
       x = x.wrapping_add(1);
        unsafe {
         //   X += 1;
           X = X.wrapping_add(1);
            Y = X;
            let _ = core::ptr::read_volatile(&Y);
            let _ = core::ptr::read_volatile(&X);
         //   assert!(x == X && X == Y);
         //   assert!(x == X && X == Y+1);
        }
    }
    
}

// Here we assume you are using `vscode` with `cortex-debug`.
//
// 0. Compile/build the example in debug (dev) mode.
//
//    > cargo build --example bare0
//    (or use the vscode build task)
//
// 1. Run the program in the debugger, let the program run for a while and
//    then press pause.
//
//    Look under Variables/Local what do you find.
//
/*   x : 630526;
     As the local variable x is inside a loop, each iteration of the loop 
     it increases its value in one unit (as it is coded as x = x+1;)
   */
//    In the Expressions (WATCH -vscode) view add X and Y
//    what do you find
//
/*  Stopping at line 45: 
    X: 630525
    Y: 630525
    If we go step by step untill line 49:
    X: 630526
    Y: 630526
*/
//    Step through one complete iteration of the loop
//    and see how the (Local) Variables are updated
//    can you foresee what will eventually happen?
//
/*
        Variable local x is increased one unit in the loop.
        For example in current iteration its value is 630526. In the next
        iteration of the loop, its value will be 630527. Next iteration will be
        630528, next 630529, etc.
        While that, the global mutable variables are also incremented. X in the same
        way as x (X = X+1) and Y is updated making its value equal to X, who has been 
        previously updated in the previous line.
        Last line of the loop just check if the booleans conditions x == X and X == Y
        are true.
*/
//    Commit your answers (bare0_1)
//
// 2. Alter the constant X_INIT so that `x += 1` directly causes `x` to wrap.
// 	  What happens when `x` wraps
//    (Hint, look under OUTPUT/Adopter Output to see the `openocd` output.)
//
//    ** your answer here **
//
//    Commit your answers (bare0_2)
//
// 3. Place a breakpoint at `x += 1`
//
//    Change (both) += operations to use wrapping_add
//    load and run the program, what happens
/*
    Usign x = x.wrapping_add(1); and X = X.wrapping_add(1)
    has the same efect as using +=, which is to increase
    the variables one unit per iteration of the loop
*/
//    Now continue execution, what happens
/*
    The only difference that I observe with previous behaviour is the fact that
    X value is updated in WATCH menu after the instruction X = X.wrapping_add(1)
    before arriving the instruction of Y = X;
*/    
//    Commit your answers (bare0_3)
//
//    (If the program did not succeed back to the breakpoint
//    you have some fault in the program and go back to 3.)
//
// 4. Change the assertion to `assert!(x == X && X == Y + 1)`, what happens?
//
/*    
    In this case, as the incrementation of X and Y do not change during the loop, when
    they arrive to the assertion X and Y have the same value so the condition X == Y+1 is not
    fulfill. Then a panic message is shown as output and the function keeps inside the infinite
    loop of the panic function
*/
//    Commit your answers (bare0_4)
//
// 5. Remove the assertion and implement "safe" functions for
//    reading and writing X and Y
//    e.g. read_x, read_y, write_x, write_y
//
//    Rewrite the program to use ONLY "safe" code besides the
//    read/write functions (which are internally "unsafe")
//
//    Commit your solution (bare0_5)
//
// 6. *Optional
//    Implement a read_u32/write_u32, taking a reference to a
//    "static" variable
//
//    Rewrite the program to use this abstraction instead of "read_x", etc.
//
//    Commit your solution (bare0_6)
//

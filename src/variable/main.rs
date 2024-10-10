/**
 * File              : main.rs
 * Author            : sandwich
 * Date              : 2024-02-20 13:26:44
 * Last Modified Date: 2024-07-08 14:59:13
 * Last Modified By  : sandwich
 */

static MY_STATIC: i32 = 1_000;
static mut MY_MUT_STATIC: i32 = 1_000;

fn main() {
    const SECOND_HOUR: usize = 3_600;
    const SECOND_DAY: usize = 24 * SECOND_HOUR;

    println!("second per day is: {SECOND_DAY}");

    {
        const SE: usize = 1_000;
        println!("{SE}"); // scope here
    }
    // println!("{SE}"); // out scope

    unsafe {
        MY_MUT_STATIC = 32;
        println!("unsafe mut static: {MY_MUT_STATIC}");
    }

    // println!("unsafe mut static: {MY_MUT_STATIC}"); // mut static must be in unsafe
    println!("{MY_STATIC}");
}
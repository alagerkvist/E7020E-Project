#![no_main]
#![no_std]
use stm32f4xx_hal::gpio::gpioa::{PA15::into_push_pull_output};
pub struct Led<'a, T>(pub &'a T)
where
    T: 'a;

macro_rules! impl_Led {
    ($G:ident) => {
        $G::into_push_pull_output();
        // impl<'a> Led<'a, $G>
        // {
        //     pub fn init(&self, w: PA15){
        //         let pa = self.0;

        //         //$G::into_push_pull_output();
        //     }
        // } 
    };
}
impl_Led!(PA15);
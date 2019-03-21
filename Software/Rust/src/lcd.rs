#![no_main]
#![no_std]

extern crate stm32f4xx_hal as hal;
use hal::prelude::_embedded_hal_digital_OutputPin as OutputPin;
use hal::prelude::_embedded_hal_blocking_spi_Write as Write;
use hal::spi::{Spi, Mode, Phase, Polarity, NoMiso};


#[derive(Debug)]
pub enum Error<E> {
    Spi(E),
}

enum Command{

}

enum Letter{
    Space,
    Percent,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Colon,
    D,
    c,
    e,
    l,
    t,
    u,
    y,
}

impl Letter {
    fn value(&self) -> &[u8] {
        match *self {
            Letter::Space => &[0,0,0,0,0,0],
            Letter::Percent => &[35, 19, 8, 100, 98, 0],
            Letter::Zero => &[62, 81, 73, 69, 62,  0],
            Letter::One => &[0, 66,127, 64,  0,  0],
            Letter::Two => &[66, 97, 81, 73, 70,  0],
            Letter::Three => &[33, 65, 73, 77, 51,  0],
            Letter::Four => &[24, 20, 18,127, 16,  0],
            Letter::Five => &[39, 69, 69, 69, 57,  0],
            Letter::Six => &[60, 74, 73, 73, 48,  0],
            Letter::Seven => &[65, 33, 17,  9,  7,  0],
            Letter::Eight => &[54, 73, 73, 73, 54,  0],
            Letter::Nine => &[6, 73, 73, 41, 30,  0],
            Letter::Colon => &[0,  0, 40,  0,  0,  0],
            Letter::D => &[127, 65, 65, 65, 62,  0],
            Letter::c => &[56, 68, 68, 68, 40,  0],
            Letter::e => &[56, 84, 84, 84, 24,  0],
            Letter::l => &[0, 65,127, 64,  0,  0,],
            Letter::t => &[4, 4, 63, 68, 36, 0],
            Letter::u => &[60, 64, 64, 32, 124, 0],
            Letter::y => &[76, 144, 144, 144, 124, 0],
        }
    }
}
/*
pub struct EA_dogs102_6w<SPI, CS, CD, RESET> {
    spi: SPI,
    cs: CS,
    cd: CD,
    reset: RESET,
}
*/
pub struct EA_dogs102_6w<SPI, CS, CD> {
    spi: SPI,
    cs: CS,
    cd: CD,
}


impl<E, SPI, CS, CD> EA_dogs102_6w<SPI, CS, CD>
where
    SPI: Write<u8, Error = E>,
    CS: OutputPin,
    CD: OutputPin,
{
    pub fn init(
        spi: SPI,
        cs: CS,
        cd: CD,
    ) -> Result<Self, Error<E>> {
        let mut ea_dogs = EA_dogs102_6w {
            spi,
            cs,
            cd,
        };
        ea_dogs.cd.set_low();
        ea_dogs.cs.set_high();
        ea_dogs.cs.set_low();
        ea_dogs.spi.write(&[0x40, 0xA1, 0xC0, 0xA4, 0xA6, 0xA2, 0x2F, 0x27, 0x81, 0x10, 0xFA, 0x90, 0xAF]);
        ea_dogs.cs.set_high();
        ea_dogs.clear_screen();
        Ok(ea_dogs)
    }

    pub fn update_duty(&mut self, duty: u32){
        self.cs.set_low();
        self.cd.set_low();
        let msb_adress = 0x10 + (0x42>>4);
        let lsb_adress = 0x00 + (0x42&0x0F);
        let adress_page = 0xB0 + (0&0x0F);
        self.spi.write(&[msb_adress, lsb_adress, adress_page]);
        self.cd.set_high();
        self.spi.write(Letter::Space.value());
        self.spi.write(Letter::Space.value());
        self.cd.set_low();
        let msb_adress = 0x10 + (0x42>>4);
        let lsb_adress = 0x00 + (0x42&0x0F);
        let adress_page = 0xB0 + (0&0x0F);
        self.spi.write(&[msb_adress, lsb_adress, adress_page]);
        self.cd.set_high();
        self.spi.write(Letter::Zero.value());
    }

    pub fn write_word(&mut self, s: &str){
        self.cd.set_low();
        self.cs.set_low();
        let msb_adress = 0x10 + (0x00>>4); //0x42 set colum address
        let lsb_adress = 0x00 + (0x00&0x0F); //Same but with the lowest bits
        let adress_page = 0xB0 + (0&0x0F);
        self.spi.write(&[msb_adress, lsb_adress, adress_page]);
        self.cd.set_high();
        for (_,letter) in s.chars().enumerate(){
            match letter {
                ' ' => { self.spi.write(Letter::Space.value()); },
                '%' => { self.spi.write(Letter::Percent.value()); }
                ':' => { self.spi.write(Letter::Colon.value()); },
                '0' => { self.spi.write(Letter::Zero.value()); },
                '1' => { self.spi.write(Letter::One.value()); },
                '2' => { self.spi.write(Letter::Two.value()); },
                '3' => { self.spi.write(Letter::Three.value()); },
                '4' => { self.spi.write(Letter::Four.value()); },
                '5' => { self.spi.write(Letter::Five.value()); },
                '6' => { self.spi.write(Letter::Six.value()); },
                '7' => { self.spi.write(Letter::Seven.value()); },
                '8' => { self.spi.write(Letter::Eight.value()); },
                '9' => { self.spi.write(Letter::Nine.value()); },
                'D' => { self.spi.write(Letter::D.value()); },
                'u' => { self.spi.write(Letter::u.value()); },
                't' => { self.spi.write(Letter::t.value()); },
                'y' => { self.spi.write(Letter::y.value()); },
                'c' => { self.spi.write(Letter::c.value()); },
                'l' => { self.spi.write(Letter::l.value()); },
                'e' => { self.spi.write(Letter::e.value()); },
                _  => {},
            };
        }
        self.cs.set_high();
    }

    pub fn clear_screen(&mut self){
        for page in 0..8 {
            //asm::bkpt();
            self.cs.set_low();
            self.cd.set_low();
            let msb_adress = 0x10 + (0>>4);
            let lsb_adress = 0x00 + (0&0x0F);
            let adress_page = 0xB0 + (page&0x0F);
            self.spi.write(&[msb_adress, lsb_adress, adress_page]);
            self.cd.set_high();
            for _ in 0..102 {
                self.spi.write(&[0x00]);
            } 
        }
    }
}
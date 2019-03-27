# Embbeded Audio Effects

E7020E Embedded System Design is a course giving at Lulea University of Technology. [Link to course syllabus](https://www.ltu.se/edu/course/E70/E7020E/E7020E-Design-av-inbyggda-system-1.67927?kursView=kursplan&l=en)

The purpose of this course is to get a understanding of embedded system and applying that knowledge into designing your own system. Our group did a system that can manipulate audio (adding effects). Where giving an audio input, add some effects and then output it on a speaker.

## Getting Started

* Start by installing Rust for embedded system, follow [this](https://rust-embedded.github.io/book/intro/install.html) guide. 
* Install OpenOCD (version 0.10.0)
* Install arm-none-eabi-gdb (version 8.2.1)
* We are using as of 2019-03-27: rustc 1.34.0-nightly (8a57831a4 2019-02-02)

### Building software
* Clone the repo ``` git clone https://github.com/alagerkvist/E7020E-Project.git ```
* If you would like to use the latest main program run ``` cargo build --release ```
* For examples, run ``` cargo build --example NAME_OF_RS_FILE (codec) ```

## Deployment

* Create FIFO file ``` mkfifo /tmp/itm.log ```
* Start your ITM trace ``` itmdump -f /tmp/itm.log -F ```
* Start OpenOCD ``` openocd -f openocd.cfg ```
* Start gdb ``` arm-none-eabi-gdb target/thumbv7em-none-eabinhf/debug/examples/NAME_OF_RS_FILE -x openocd.gdb ```

## Authors

* **Christoffer Forsberg** - *PCB design*
* **Anders Lagerkvist** - *Programming*
* **Gustav Larsson** - *Programming* 

## Grades
**3**
* Control lamp with PWM signal through UART/potentiometer/button

* **Christoffer Forsberg** - *PWM signal*
* **Anders Lagerkvist** - *ADC for potentiometer*
* **Gustav Larsson** - *Button and UART*

**4**
* Store input siganl in RAM with the help of a button
* Get a sinus wave to output to the codec
* Do an echo with the "signal"
* Show settings on LCD
* Implement I2S/(I2C/SPI) for audio codec

* **Christoffer Forsberg** - *Signal processing, store audio input to RAM with button*
* **Anders Lagerkvist** - *LCD, I2S sine wave output to codec*
* **Gustav Larsson** - *LCD I2C/SPI*

**5**
* Overlay two signals as output
* Integrate different impulse for signal proccessing
* Use an USB as storage

* **Christoffer Forsberg** - *Real time signal processing*
* **Anders Lagerkvist** - *USB implementation*
* **Gustav Larsson** - *Real time signal processing*


## Implemented features

* PWM/LED
* Button
* LCD
* Codec (ish)
* ADC

## Known issues and limitation

* MCUs adc can only read from one potentiometer
* LCD screen has no library yet, so it's a pain in the a** to write to it
* As of now, the codec is inconsistent when writing to it, possible reason to this can be impedance mismatch or wrongfully clock generation 
* PCB layout for regulation is not right implemented
* Footprint for mini-usb type B should be the other way around (no possibility to mount it on the PCB as of now)
* Codec is in master mode, and should be in slave mode (can be fixed with a pull down resistor instead of a pull up resistor)
* Layout for the potentiometer is too close, needs to seperate more
* Footprint for RCA connector needs to be bigger in their NPTH hole 

## Future work

* Implement ADC so it can be read from three potentiometers
* Library for LCD
* Successfully send data to codec
* Store data from codec into ram
* Sound processing
* Store/read data on USB 

## Acknowledgments
PhD Student Cristina Paniagua [Link to profile](https://www.ltu.se/staff/c/cripan-1.167417?l=en)

Professor Per Lindgren [Link to profile](https://www.ltu.se/staff/p/pln-1.11258?l=en)

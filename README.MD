# Embbeded Audio Effects

E7020E Embedded System Design is a course giving at Lulea University of Technology. [Link to course syllabus](https://www.ltu.se/edu/course/E70/E7020E/E7020E-Design-av-inbyggda-system-1.67927?kursView=kursplan&l=en)

The purpose of this course is to get a understanding of embedded system and applying that knowledge into designing your own system. Our group did a system that can manipulate audio (adding effects). Where giving an audio input, add some effects and then output it on a speaker.

## Getting Started

TBD

### Prerequisites

TBD
```
TBD
```

### Installing

TBD

```
TBD
```

## Deployment

rustc 1.34.0-nightly (8a57831a4 2019-02-02)
## Built With

* Cargo
* OpenOCD
* arm-none-eabi-gdb

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

## Acknowledgments
PhD Student Cristina Paniagua [Link to profile](https://www.ltu.se/staff/c/cripan-1.167417?l=en)
Professor Per Lindgren [Link to profile](https://www.ltu.se/staff/p/pln-1.11258?l=en)

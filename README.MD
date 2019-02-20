# Embbeded Audio Effects

Design and build a PCB that manipulate audio (adding effects). Rust is used to program the MCU. 

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

TBD
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
TBD

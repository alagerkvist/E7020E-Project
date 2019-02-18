EESchema Schematic File Version 4
LIBS:Lab1-cache
EELAYER 26 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 2 2
Title ""
Date ""
Rev ""
Comp ""
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
$Comp
L Lab1-rescue:R-Device R?
U 1 1 5C74F3FA
P 900 4350
AR Path="/5C74F3FA" Ref="R?"  Part="1" 
AR Path="/5C725C26/5C74F3FA" Ref="R23"  Part="1" 
F 0 "R23" H 830 4396 50  0000 R CNN
F 1 "10k" H 830 4305 50  0000 R CNN
F 2 "Resistor_SMD:R_0805_2012Metric_Pad1.15x1.40mm_HandSolder" V 830 4350 50  0001 C CNN
F 3 "~" H 900 4350 50  0001 C CNN
	1    900  4350
	-1   0    0    -1  
$EndComp
$Comp
L Lab1-rescue:C-Device C?
U 1 1 5C74F401
P 1400 4250
AR Path="/5C74F401" Ref="C?"  Part="1" 
AR Path="/5C725C26/5C74F401" Ref="C33"  Part="1" 
F 0 "C33" H 1515 4296 50  0000 L CNN
F 1 "100n" H 1515 4205 50  0000 L CNN
F 2 "Capacitor_SMD:C_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 1438 4100 50  0001 C CNN
F 3 "~" H 1400 4250 50  0001 C CNN
	1    1400 4250
	1    0    0    -1  
$EndComp
$Comp
L Lab1-rescue:R-Device R?
U 1 1 5C74F408
P 1150 4050
AR Path="/5C74F408" Ref="R?"  Part="1" 
AR Path="/5C725C26/5C74F408" Ref="R24"  Part="1" 
F 0 "R24" H 1080 4096 50  0000 R CNN
F 1 "1k" H 1080 4005 50  0000 R CNN
F 2 "Resistor_SMD:R_0805_2012Metric_Pad1.15x1.40mm_HandSolder" V 1080 4050 50  0001 C CNN
F 3 "~" H 1150 4050 50  0001 C CNN
	1    1150 4050
	0    1    -1   0   
$EndComp
Wire Wire Line
	900  3400 900  3550
Wire Wire Line
	900  3950 900  4050
Connection ~ 1400 4050
Wire Wire Line
	1400 4050 1400 4100
Wire Wire Line
	1400 4650 1400 4400
Wire Wire Line
	1400 4050 1600 4050
$Comp
L Lab1-rescue:SW_Push-Switch SW?
U 1 1 5C74F422
P 900 3750
AR Path="/5C74F422" Ref="SW?"  Part="1" 
AR Path="/5C725C26/5C74F422" Ref="SW1"  Part="1" 
F 0 "SW1" V 946 3702 50  0000 R CNN
F 1 "SW_Push" V 855 3702 50  0000 R CNN
F 2 "Button_Switch_SMD:SW_SPST_B3S-1000" H 900 3950 50  0001 C CNN
F 3 "" H 900 3950 50  0001 C CNN
	1    900  3750
	0    -1   -1   0   
$EndComp
Wire Wire Line
	900  4050 900  4200
Wire Wire Line
	900  4500 900  4650
Wire Wire Line
	1000 4050 900  4050
Wire Wire Line
	1300 4050 1400 4050
Connection ~ 900  4050
$Comp
L Lab1-rescue:R-Device R?
U 1 1 5C74F434
P 2150 4300
AR Path="/5C74F434" Ref="R?"  Part="1" 
AR Path="/5C725C26/5C74F434" Ref="R25"  Part="1" 
F 0 "R25" H 2080 4346 50  0000 R CNN
F 1 "10k" H 2080 4255 50  0000 R CNN
F 2 "Resistor_SMD:R_0805_2012Metric_Pad1.15x1.40mm_HandSolder" V 2080 4300 50  0001 C CNN
F 3 "~" H 2150 4300 50  0001 C CNN
	1    2150 4300
	-1   0    0    -1  
$EndComp
$Comp
L Lab1-rescue:C-Device C?
U 1 1 5C74F43B
P 2650 4200
AR Path="/5C74F43B" Ref="C?"  Part="1" 
AR Path="/5C725C26/5C74F43B" Ref="C35"  Part="1" 
F 0 "C35" H 2765 4246 50  0000 L CNN
F 1 "100n" H 2765 4155 50  0000 L CNN
F 2 "Capacitor_SMD:C_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 2688 4050 50  0001 C CNN
F 3 "~" H 2650 4200 50  0001 C CNN
	1    2650 4200
	1    0    0    -1  
$EndComp
$Comp
L Lab1-rescue:R-Device R?
U 1 1 5C74F442
P 2400 4000
AR Path="/5C74F442" Ref="R?"  Part="1" 
AR Path="/5C725C26/5C74F442" Ref="R26"  Part="1" 
F 0 "R26" H 2330 4046 50  0000 R CNN
F 1 "1k" H 2330 3955 50  0000 R CNN
F 2 "Resistor_SMD:R_0805_2012Metric_Pad1.15x1.40mm_HandSolder" V 2330 4000 50  0001 C CNN
F 3 "~" H 2400 4000 50  0001 C CNN
	1    2400 4000
	0    1    -1   0   
$EndComp
Wire Wire Line
	2150 3900 2150 4000
Connection ~ 2650 4000
Wire Wire Line
	2650 4000 2650 4050
Wire Wire Line
	2650 4600 2650 4350
Wire Wire Line
	2650 4000 2850 4000
$Comp
L Lab1-rescue:SW_Push-Switch SW?
U 1 1 5C74F45C
P 2150 3700
AR Path="/5C74F45C" Ref="SW?"  Part="1" 
AR Path="/5C725C26/5C74F45C" Ref="SW2"  Part="1" 
F 0 "SW2" V 2196 3652 50  0000 R CNN
F 1 "SW_Push" V 2105 3652 50  0000 R CNN
F 2 "Button_Switch_SMD:SW_SPST_B3S-1000" H 2150 3900 50  0001 C CNN
F 3 "" H 2150 3900 50  0001 C CNN
	1    2150 3700
	0    -1   -1   0   
$EndComp
Wire Wire Line
	2150 4000 2150 4150
Wire Wire Line
	2150 4450 2150 4600
Wire Wire Line
	2250 4000 2150 4000
Wire Wire Line
	2550 4000 2650 4000
Connection ~ 2150 4000
$Comp
L Lab1-rescue:R-Device R?
U 1 1 5C74F46E
P 3450 4250
AR Path="/5C74F46E" Ref="R?"  Part="1" 
AR Path="/5C725C26/5C74F46E" Ref="R30"  Part="1" 
F 0 "R30" H 3380 4296 50  0000 R CNN
F 1 "10k" H 3380 4205 50  0000 R CNN
F 2 "Resistor_SMD:R_0805_2012Metric_Pad1.15x1.40mm_HandSolder" V 3380 4250 50  0001 C CNN
F 3 "~" H 3450 4250 50  0001 C CNN
	1    3450 4250
	-1   0    0    -1  
$EndComp
$Comp
L Lab1-rescue:C-Device C?
U 1 1 5C74F475
P 3950 4150
AR Path="/5C74F475" Ref="C?"  Part="1" 
AR Path="/5C725C26/5C74F475" Ref="C36"  Part="1" 
F 0 "C36" H 4065 4196 50  0000 L CNN
F 1 "100n" H 4065 4105 50  0000 L CNN
F 2 "Capacitor_SMD:C_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 3988 4000 50  0001 C CNN
F 3 "~" H 3950 4150 50  0001 C CNN
	1    3950 4150
	1    0    0    -1  
$EndComp
$Comp
L Lab1-rescue:R-Device R?
U 1 1 5C74F47C
P 3700 3950
AR Path="/5C74F47C" Ref="R?"  Part="1" 
AR Path="/5C725C26/5C74F47C" Ref="R33"  Part="1" 
F 0 "R33" H 3630 3996 50  0000 R CNN
F 1 "1k" H 3630 3905 50  0000 R CNN
F 2 "Resistor_SMD:R_0805_2012Metric_Pad1.15x1.40mm_HandSolder" V 3630 3950 50  0001 C CNN
F 3 "~" H 3700 3950 50  0001 C CNN
	1    3700 3950
	0    1    -1   0   
$EndComp
Wire Wire Line
	3450 3850 3450 3950
Connection ~ 3950 3950
Wire Wire Line
	3950 3950 3950 4000
Wire Wire Line
	3950 4550 3950 4300
Wire Wire Line
	3950 3950 4150 3950
$Comp
L Lab1-rescue:SW_Push-Switch SW?
U 1 1 5C74F496
P 3450 3650
AR Path="/5C74F496" Ref="SW?"  Part="1" 
AR Path="/5C725C26/5C74F496" Ref="SW3"  Part="1" 
F 0 "SW3" V 3496 3602 50  0000 R CNN
F 1 "SW_Push" V 3405 3602 50  0000 R CNN
F 2 "Button_Switch_SMD:SW_SPST_B3S-1000" H 3450 3850 50  0001 C CNN
F 3 "" H 3450 3850 50  0001 C CNN
	1    3450 3650
	0    -1   -1   0   
$EndComp
Wire Wire Line
	3450 3950 3450 4100
Wire Wire Line
	3450 4400 3450 4550
Wire Wire Line
	3550 3950 3450 3950
Wire Wire Line
	3850 3950 3950 3950
Connection ~ 3450 3950
$Comp
L Lab1-rescue:R-Device R?
U 1 1 5C79763F
P 6000 4300
AR Path="/5C79763F" Ref="R?"  Part="1" 
AR Path="/5C725C26/5C79763F" Ref="R34"  Part="1" 
F 0 "R34" H 5930 4346 50  0000 R CNN
F 1 "10k" H 5930 4255 50  0000 R CNN
F 2 "Resistor_SMD:R_0805_2012Metric_Pad1.15x1.40mm_HandSolder" V 5930 4300 50  0001 C CNN
F 3 "~" H 6000 4300 50  0001 C CNN
	1    6000 4300
	-1   0    0    -1  
$EndComp
$Comp
L Lab1-rescue:C-Device C?
U 1 1 5C797646
P 6500 4200
AR Path="/5C797646" Ref="C?"  Part="1" 
AR Path="/5C725C26/5C797646" Ref="C42"  Part="1" 
F 0 "C42" H 6615 4246 50  0000 L CNN
F 1 "100n" H 6615 4155 50  0000 L CNN
F 2 "Capacitor_SMD:C_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 6538 4050 50  0001 C CNN
F 3 "~" H 6500 4200 50  0001 C CNN
	1    6500 4200
	1    0    0    -1  
$EndComp
$Comp
L Lab1-rescue:R-Device R?
U 1 1 5C79764D
P 6250 4000
AR Path="/5C79764D" Ref="R?"  Part="1" 
AR Path="/5C725C26/5C79764D" Ref="R35"  Part="1" 
F 0 "R35" H 6180 4046 50  0000 R CNN
F 1 "1k" H 6180 3955 50  0000 R CNN
F 2 "Resistor_SMD:R_0805_2012Metric_Pad1.15x1.40mm_HandSolder" V 6180 4000 50  0001 C CNN
F 3 "~" H 6250 4000 50  0001 C CNN
	1    6250 4000
	0    1    -1   0   
$EndComp
Wire Wire Line
	6000 3900 6000 4000
Connection ~ 6500 4000
Wire Wire Line
	6500 4000 6500 4050
$Comp
L Lab1-rescue:GNDD-power #PWR?
U 1 1 5C797658
P 6500 4600
AR Path="/5C797658" Ref="#PWR?"  Part="1" 
AR Path="/5C725C26/5C797658" Ref="#PWR051"  Part="1" 
F 0 "#PWR051" H 6500 4350 50  0001 C CNN
F 1 "GNDD" H 6504 4445 50  0000 C CNN
F 2 "" H 6500 4600 50  0001 C CNN
F 3 "" H 6500 4600 50  0001 C CNN
	1    6500 4600
	1    0    0    -1  
$EndComp
$Comp
L Lab1-rescue:GNDD-power #PWR?
U 1 1 5C79765E
P 6000 4600
AR Path="/5C79765E" Ref="#PWR?"  Part="1" 
AR Path="/5C725C26/5C79765E" Ref="#PWR047"  Part="1" 
F 0 "#PWR047" H 6000 4350 50  0001 C CNN
F 1 "GNDD" H 6004 4445 50  0000 C CNN
F 2 "" H 6000 4600 50  0001 C CNN
F 3 "" H 6000 4600 50  0001 C CNN
	1    6000 4600
	1    0    0    -1  
$EndComp
Wire Wire Line
	6500 4600 6500 4350
Wire Wire Line
	6500 4000 6700 4000
$Comp
L Lab1-rescue:SW_Push-Switch SW?
U 1 1 5C797666
P 6000 3700
AR Path="/5C797666" Ref="SW?"  Part="1" 
AR Path="/5C725C26/5C797666" Ref="SW5"  Part="1" 
F 0 "SW5" V 6046 3652 50  0000 R CNN
F 1 "SW_Push" V 5955 3652 50  0000 R CNN
F 2 "Button_Switch_SMD:SW_SPST_B3S-1000" H 6000 3900 50  0001 C CNN
F 3 "" H 6000 3900 50  0001 C CNN
	1    6000 3700
	0    -1   -1   0   
$EndComp
Wire Wire Line
	6000 4000 6000 4150
Wire Wire Line
	6000 4450 6000 4600
Wire Wire Line
	6100 4000 6000 4000
Wire Wire Line
	6400 4000 6500 4000
Connection ~ 6000 4000
$Comp
L Lab1-rescue:R-Device R?
U 1 1 5C797678
P 4750 4250
AR Path="/5C797678" Ref="R?"  Part="1" 
AR Path="/5C725C26/5C797678" Ref="R1"  Part="1" 
F 0 "R1" H 4680 4296 50  0000 R CNN
F 1 "10k" H 4680 4205 50  0000 R CNN
F 2 "Resistor_SMD:R_0805_2012Metric_Pad1.15x1.40mm_HandSolder" V 4680 4250 50  0001 C CNN
F 3 "~" H 4750 4250 50  0001 C CNN
	1    4750 4250
	-1   0    0    -1  
$EndComp
$Comp
L Lab1-rescue:C-Device C?
U 1 1 5C79767F
P 5250 4150
AR Path="/5C79767F" Ref="C?"  Part="1" 
AR Path="/5C725C26/5C79767F" Ref="C19"  Part="1" 
F 0 "C19" H 5365 4196 50  0000 L CNN
F 1 "100n" H 5365 4105 50  0000 L CNN
F 2 "Capacitor_SMD:C_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 5288 4000 50  0001 C CNN
F 3 "~" H 5250 4150 50  0001 C CNN
	1    5250 4150
	1    0    0    -1  
$EndComp
$Comp
L Lab1-rescue:R-Device R?
U 1 1 5C797686
P 5000 3950
AR Path="/5C797686" Ref="R?"  Part="1" 
AR Path="/5C725C26/5C797686" Ref="R22"  Part="1" 
F 0 "R22" H 4930 3996 50  0000 R CNN
F 1 "1k" H 4930 3905 50  0000 R CNN
F 2 "Resistor_SMD:R_0805_2012Metric_Pad1.15x1.40mm_HandSolder" V 4930 3950 50  0001 C CNN
F 3 "~" H 5000 3950 50  0001 C CNN
	1    5000 3950
	0    1    -1   0   
$EndComp
Wire Wire Line
	4750 3850 4750 3950
Connection ~ 5250 3950
Wire Wire Line
	5250 3950 5250 4000
$Comp
L Lab1-rescue:GNDD-power #PWR?
U 1 1 5C797691
P 5250 4550
AR Path="/5C797691" Ref="#PWR?"  Part="1" 
AR Path="/5C725C26/5C797691" Ref="#PWR010"  Part="1" 
F 0 "#PWR010" H 5250 4300 50  0001 C CNN
F 1 "GNDD" H 5254 4395 50  0000 C CNN
F 2 "" H 5250 4550 50  0001 C CNN
F 3 "" H 5250 4550 50  0001 C CNN
	1    5250 4550
	1    0    0    -1  
$EndComp
$Comp
L Lab1-rescue:GNDD-power #PWR?
U 1 1 5C797697
P 4750 4550
AR Path="/5C797697" Ref="#PWR?"  Part="1" 
AR Path="/5C725C26/5C797697" Ref="#PWR06"  Part="1" 
F 0 "#PWR06" H 4750 4300 50  0001 C CNN
F 1 "GNDD" H 4754 4395 50  0000 C CNN
F 2 "" H 4750 4550 50  0001 C CNN
F 3 "" H 4750 4550 50  0001 C CNN
	1    4750 4550
	1    0    0    -1  
$EndComp
Wire Wire Line
	5250 4550 5250 4300
Wire Wire Line
	5250 3950 5450 3950
$Comp
L Lab1-rescue:SW_Push-Switch SW?
U 1 1 5C7976A0
P 4750 3650
AR Path="/5C7976A0" Ref="SW?"  Part="1" 
AR Path="/5C725C26/5C7976A0" Ref="SW4"  Part="1" 
F 0 "SW4" V 4796 3602 50  0000 R CNN
F 1 "SW_Push" V 4705 3602 50  0000 R CNN
F 2 "Button_Switch_SMD:SW_SPST_B3S-1000" H 4750 3850 50  0001 C CNN
F 3 "" H 4750 3850 50  0001 C CNN
	1    4750 3650
	0    -1   -1   0   
$EndComp
Wire Wire Line
	4750 3950 4750 4100
Wire Wire Line
	4750 4400 4750 4550
Wire Wire Line
	4850 3950 4750 3950
Wire Wire Line
	5150 3950 5250 3950
Connection ~ 4750 3950
$Comp
L Lab1-rescue:GNDA-power #PWR?
U 1 1 5C7DE3CB
P 900 4650
AR Path="/5C7DE3CB" Ref="#PWR?"  Part="1" 
AR Path="/5C725C26/5C7DE3CB" Ref="#PWR0109"  Part="1" 
F 0 "#PWR0109" H 900 4400 50  0001 C CNN
F 1 "GNDA" H 905 4477 50  0000 C CNN
F 2 "" H 900 4650 50  0001 C CNN
F 3 "" H 900 4650 50  0001 C CNN
	1    900  4650
	1    0    0    -1  
$EndComp
$Comp
L Lab1-rescue:GNDA-power #PWR?
U 1 1 5C7DEAED
P 1400 4650
AR Path="/5C7DEAED" Ref="#PWR?"  Part="1" 
AR Path="/5C725C26/5C7DEAED" Ref="#PWR0111"  Part="1" 
F 0 "#PWR0111" H 1400 4400 50  0001 C CNN
F 1 "GNDA" H 1405 4477 50  0000 C CNN
F 2 "" H 1400 4650 50  0001 C CNN
F 3 "" H 1400 4650 50  0001 C CNN
	1    1400 4650
	1    0    0    -1  
$EndComp
$Comp
L Lab1-rescue:GNDA-power #PWR?
U 1 1 5C7DEB20
P 2150 4600
AR Path="/5C7DEB20" Ref="#PWR?"  Part="1" 
AR Path="/5C725C26/5C7DEB20" Ref="#PWR0112"  Part="1" 
F 0 "#PWR0112" H 2150 4350 50  0001 C CNN
F 1 "GNDA" H 2155 4427 50  0000 C CNN
F 2 "" H 2150 4600 50  0001 C CNN
F 3 "" H 2150 4600 50  0001 C CNN
	1    2150 4600
	1    0    0    -1  
$EndComp
$Comp
L Lab1-rescue:GNDA-power #PWR?
U 1 1 5C7DEC6B
P 2650 4600
AR Path="/5C7DEC6B" Ref="#PWR?"  Part="1" 
AR Path="/5C725C26/5C7DEC6B" Ref="#PWR0113"  Part="1" 
F 0 "#PWR0113" H 2650 4350 50  0001 C CNN
F 1 "GNDA" H 2655 4427 50  0000 C CNN
F 2 "" H 2650 4600 50  0001 C CNN
F 3 "" H 2650 4600 50  0001 C CNN
	1    2650 4600
	1    0    0    -1  
$EndComp
$Comp
L Lab1-rescue:GNDA-power #PWR?
U 1 1 5C7DEC9E
P 3450 4550
AR Path="/5C7DEC9E" Ref="#PWR?"  Part="1" 
AR Path="/5C725C26/5C7DEC9E" Ref="#PWR0114"  Part="1" 
F 0 "#PWR0114" H 3450 4300 50  0001 C CNN
F 1 "GNDA" H 3455 4377 50  0000 C CNN
F 2 "" H 3450 4550 50  0001 C CNN
F 3 "" H 3450 4550 50  0001 C CNN
	1    3450 4550
	1    0    0    -1  
$EndComp
$Comp
L Lab1-rescue:GNDA-power #PWR?
U 1 1 5C7DEDAD
P 3950 4550
AR Path="/5C7DEDAD" Ref="#PWR?"  Part="1" 
AR Path="/5C725C26/5C7DEDAD" Ref="#PWR0115"  Part="1" 
F 0 "#PWR0115" H 3950 4300 50  0001 C CNN
F 1 "GNDA" H 3955 4377 50  0000 C CNN
F 2 "" H 3950 4550 50  0001 C CNN
F 3 "" H 3950 4550 50  0001 C CNN
	1    3950 4550
	1    0    0    -1  
$EndComp
Wire Wire Line
	900  3400 1150 3400
Wire Wire Line
	6000 3400 6000 3500
Wire Wire Line
	4750 3450 4750 3400
Connection ~ 4750 3400
Wire Wire Line
	4750 3400 6000 3400
Wire Wire Line
	3450 3450 3450 3400
Connection ~ 3450 3400
Wire Wire Line
	3450 3400 4750 3400
Wire Wire Line
	2150 3500 2150 3400
Connection ~ 2150 3400
Wire Wire Line
	2150 3400 3450 3400
Wire Wire Line
	1050 3150 1150 3150
Wire Wire Line
	1150 3150 1150 3400
Connection ~ 1150 3400
Wire Wire Line
	1150 3400 2150 3400
Text HLabel 1050 3150 0    50   Input ~ 0
3.3v
Text HLabel 6700 4000 2    50   Output ~ 0
Button5
Text HLabel 5450 3950 2    50   Output ~ 0
Button4
Text HLabel 4150 3950 2    50   Output ~ 0
Button3
Text HLabel 2850 4000 2    50   Output ~ 0
Button2
Text HLabel 1600 4050 2    50   Output ~ 0
Button1
$EndSCHEMATC

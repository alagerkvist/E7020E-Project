close all;clear all;clc
%read in song
load gong.mat;

fDelayInPercent = 0.5; 
iSamplingsFrequency = 8192; %Sampling frequency
fOriginalSoundHardness = 0.1;
fImpulseHardness = 0.5;
iTotalNumberOfSampels=length(y); %Length of input signal. 
aSoundVector=y(:,1); %Vector form/2D version of length 
iDelayInSamples = round(fDelayInPercent*iTotalNumberOfSampels); %Delay 
 
iDelayedSignalSampels = length(y); %Length of the delayed signal. 
aImpulseVector = zeros(length(y),1); %Impulse vector
iImpulseVector(1) = fOriginalSoundHardness; 
iImpulseVector(iDelayInSamples)= fImpulseHardness ; %Impulse at delay 
aDelayedSound = conv(aSoundVector,iImpulseVector); %Convolution


%plots
subplot(3,1,1);
plot(y); %Plot of original voice 
subplot(3,1,2);
plot(iImpulseVector); %plot of impulse  
subplot(3,1,3); 
plot(aDelayedSound); %Plot of Echoed sound 
 
sound(aDelayedSound,iSamplingsFrequency); %Echoed sound



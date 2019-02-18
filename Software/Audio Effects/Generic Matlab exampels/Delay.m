close all;clear all;clc
%read in song
load gong.mat;

fDelayInPercent = 0.5; 
iSamplingsFrequency = 8192; %Sampling frequency
fOriginalSoundHardness = 0;
fImpulseHardness = 0.5;
iTotalNumberOfSampels=length(y); %Length of input signal. 
aSoundVector=y(:,1); %Vector form/2D version of length 
iDelayInSamples = round(fDelayInPercent*iTotalNumberOfSampels); %Delay 
  
aImpulseVector = zeros(length(y),1); %Impulse vector
aImpulseVector(1) = fOriginalSoundHardness; 
aImpulseVector(iDelayInSamples)= fImpulseHardness ; %Impulse at delay 
aDelayedSound = conv(aSoundVector,aImpulseVector); %Convolution


%plots
subplot(3,1,1);
plot(y); %Plot of original voice 
title('Original sound');
ylabel('Amplitude');
xlabel('Sampel');

subplot(3,1,2);
plot(aImpulseVector); %plot of impulse  
title('Impulse answer');
ylabel('Amplitude');
xlabel('Sampel');

subplot(3,1,3); 
plot(aDelayedSound); %Plot of Echoed sound 
title('Output sound');
ylabel('Amplitude');
xlabel('Sampel');


sound(aDelayedSound,iSamplingsFrequency); %Echoed sound



# music

Plays a sound from assets when an action occurs.

Designed to work with https://github.com/Vrixyz/button for connection over bluetooth.

# Bluetooth
To setup bluetooth on a raspberry pi headless is not that easy, here is my setup:
- ~/.asoundrc
```
defaults.bluealsa.device "5C:49:7D:88:06:39"
defaults.bluealsa.profile "a2dp"
defaults.bluealsa.delay 10000
pcm.btreceiver { 
    type plug 
    slave.pcm { 
        type bluealsa 
        device "5C:49:7D:88:06:39"
        profile "a2dp" 
    } 
    hint { 
        show on 
        description "Bluetooth Receiver" 
    }
}
pcm.!default {
    type plug
    slave.pcm "btreceiver"
}
```
- I had to install `blue-alsa` for playing music and `pulseaudio` to communicate with my soundbar.
- I connected to bluetooth devices (button + soundbar) via bluetoothctl
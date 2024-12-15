# Result

```bash
# 소리는 나오지 않고 분석만
$ cargo r No\ Hero\ -\ On\ My\ Mind\ \[NCS\ Release\].mp3

Capturing 3 seconds... Please rock!
Capture Spec = AudioSpec { freq: 22050, format: S16LSB, channels: 2, silence: 0, samples: 1024, size: 4096 }
AudioDriver: "coreaudio"
Average Volume of your Recording = 0.052986592%
Max Volume of your Recording = 0.35706657%
Playing...
Playback Spec = AudioSpec { freq: 22050, format: S16LSB, channels: 2, silence: 0, samples: 1024, size: 4096 }


# 아주 긴 삐~~음 시끄럽다.
$ cargo r --bin audio-squarewave

AudioSpec { freq: 44100, format: F32LSB, channels: 1, silence: 0, samples: 2048, size: 8192 }

```


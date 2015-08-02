# rust-software-render [![Build Status](https://travis-ci.org/ReanGD/rust-software-render.svg?branch=master)](https://travis-ci.org/ReanGD/rust-software-render)

Для сборки проекта у вас должен стоять компилятор [Rust](http://www.rust-lang.org/), менеджер пакетов для него [Cargo](https://crates.io/).

Так же дополнительно требуется установить библиотеку [sdl2](https://www.libsdl.org/download-2.0.php).

Для arch-linux последняя ставится вот так:
```Shell
sudo pacman -S sdl2
```
Для ubuntu вот так:
```Shell
sudo add-apt-repository ppa:team-xbmc/ppa -y
sudo apt-get update -q
sudo apt-get install libsdl2-dev
```

Собрать и запустить проект можно, выполнив такую команду из корня проекта:
```Shell
cargo run --release
```

Скриншоты:
![Растеризация на плоскости](https://github.com/ReanGD/rust-software-render/blob/master/screenshots/scene_1.png "Растеризация на плоскости")

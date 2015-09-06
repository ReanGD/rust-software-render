# rust-software-render 

[v0.1](https://github.com/ReanGD/rust-software-render/tree/v01) [![Build Status](https://travis-ci.org/ReanGD/rust-software-render.svg?branch=v01)](https://travis-ci.org/ReanGD/rust-software-render) 
[v0.2](https://github.com/ReanGD/rust-software-render/tree/v02) [![Build Status](https://travis-ci.org/ReanGD/rust-software-render.svg?branch=v02)](https://travis-ci.org/ReanGD/rust-software-render) 
[v0.3](https://github.com/ReanGD/rust-software-render/tree/v03) [![Build Status](https://travis-ci.org/ReanGD/rust-software-render.svg?branch=v03)](https://travis-ci.org/ReanGD/rust-software-render) 
[master](https://github.com/ReanGD/rust-software-render/tree/master) [![Build Status](https://travis-ci.org/ReanGD/rust-software-render.svg?branch=master)](https://travis-ci.org/ReanGD/rust-software-render)

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

[v0.1](https://github.com/ReanGD/rust-software-render/tree/v01) ([статья](http://reangdblog.blogspot.com/2015/08/software-render-rust.html)):

16 тыс. случайных треугольников на экране и всего 1 fps:
![Растеризация на плоскости](https://github.com/ReanGD/rust-software-render/blob/master/screenshots/scene_1.png "Растеризация на плоскости")

[v0.2](https://github.com/ReanGD/rust-software-render/tree/v02) ([статья](http://reangdblog.blogspot.com/2015/09/software-render-rust-3d.html)):

Низкополигональная модель освещение даже не повершинное, а пограневое, чуть более 2 млн triangle per second:
![Кольцо и пограневое освещение](https://github.com/ReanGD/rust-software-render/blob/v02/screenshots/scene_2_1.png "Кольцо и пограневое освещение")

Высокополигональная модель освещение такое же, tps = 3.3 млн:
![Высокополигональный монстр](https://github.com/ReanGD/rust-software-render/blob/v02/screenshots/scene_2_2.png "Высокополигональный монстр")

[v0.3](https://github.com/ReanGD/rust-software-render/tree/v03):

# rust-software-render

[v0.1](https://github.com/ReanGD/rust-software-render/tree/v01) [![Build Status](https://travis-ci.org/ReanGD/rust-software-render.svg?branch=v01)](https://travis-ci.org/ReanGD/rust-software-render) [![Build status](https://ci.appveyor.com/api/projects/status/y32wy5vu51q9hscm/branch/v01?svg=true)](https://ci.appveyor.com/project/ReanGD/rust-software-render/branch/v01)

Подготовка окружения
--
Для сборки проекта у вас должны стоять:
- компилятор [Rust](http://www.rust-lang.org/) (для Windows тестируется только stable версия, для Linux: nightly, beta, stable)
- менеджер пакетов для rust - [Cargo](https://crates.io/)

Для Windows необходимо добавить папку с rustc и cargo в PATH

Дополнительные пакеты
--
Требуется установить библиотеку [sdl2](https://www.libsdl.org/download-2.0.php).

Для Arch-Linux ставится вот так:
```Shell
sudo pacman -S sdl2
```
Для Ubuntu вот так:
```Shell
sudo add-apt-repository ppa:zoogie/sdl2-snapshots -y
sudo apt-get update -q
sudo apt-get install libsdl2-dev
```

Для Windows:
- качаем [SDL2-devel-2.0.4-mingw.tar.gz](https://www.libsdl.org/release/SDL2-devel-2.0.4-mingw.tar.gz), распаковываем, ищем файл SDL2-2.0.4\TARGET-w64-mingw32\lib\libSDL2.dll.a и копируем его в RUST_PATH\lib\rustlib\TARGET-pc-windows-gnu\lib\
- качаем [SDL2-2.0.4-win32-x86.zip](https://www.libsdl.org/release/SDL2-2.0.4-win32-x86.zip) или [SDL2-2.0.4-win32-x64.zip](https://www.libsdl.org/release/SDL2-2.0.4-win32-x64.zip) в зависимости от платформы, распаковываем и кладем либо в директорию, которая находится в PATH, либо рядом с exe который получится после компиляции проекта
- для x86 платформы качаем [gcc-core-4.8.1-4-mingw32-dll.tar.lzma](http://sourceforge.net/projects/mingw/files/MinGW/Base/gcc/Version4/gcc-4.8.1-4/gcc-core-4.8.1-4-mingw32-dll.tar.lzma), распоковываем и libgcc_s_dw2-1.dll кладем либо в директорию, которая находится в PATH, либо рядом с exe который получится после компиляции проекта

Готовые сборки:
--
В связи со сложным процессом установки под Windows - можно просто [взять](https://github.com/ReanGD/rust-software-render/releases) готовые сборки для платформ x86 и x64, для каждой версии приложения.

Сборка и запуск
--
Из корня проекта выполняем:
```Shell
cargo build --release
```
Если нужно сразу скопилировать и запустить, то так:
```Shell
cargo run --release
```


Скриншоты
--

[v0.1](https://github.com/ReanGD/rust-software-render/tree/v01) ([статья](http://reangdblog.blogspot.com/2015/08/software-render-rust.html)):

16 тыс. случайных треугольников на экране и всего 1 fps:
![Растеризация на плоскости](https://github.com/ReanGD/rust-software-render/blob/v01/screenshots/scene_1.png "Растеризация на плоскости")

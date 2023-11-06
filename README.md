# flutter_rust_example

Calling Rust Code from Flutter: A Guide to FFI Integration
![IMG_EB52EC3960F8-1](https://github.com/Tristan-2021/rust_flutter_example/assets/79274889/ca5b3b05-45b4-42ed-ae33-8cbeccff0a63)

## Empezemos 
Primeramente necesitamos tener instaldo rust, en mi caso uso "cargo 1.73.0", puede usar este comando: 
>  flutter pub get

>  cargo --version
Luego de ello puede clonar el proyecto y dirigirse al archivo native: 
> cd native
> cargo install flutter_rust_bridge_codegen@1.80.1

Una vez allí  necesitar usar este comandos : 
> cargo build
Ahora necesitamos generar los puentes: 
> flutter_rust_bridge_codegen -r src/api.rs -d ../lib/bridge_generated.dart --dart-decl-output ../lib/bridge_definitions.dart
Corremos est comando desde la carpeta raiz
> cargo install cargo-lipo
Luego nos vamos otra vez a a la carpeta native  y corremos este comando
> cargo lipo
 Lo que hara este comando es instalar las arquitecturas para correr en nuestros dipositovs IOS, ahora necesitamos copiar la arquitectura dentro de la carpeta nativa 
 con este comando 
> cp target/universal/debug/librustflutter.a ../ios/Runner
Otra vez necesitamos correr los encabezados
> flutter_rust_bridge_codegen -r native/src/api.rs -d lib/bridge_generated.dart --dart-decl-output lib/bridge_definitions.dart -c ios/Runner/bridge_generated.h
Ojo le saldra un error en el archivo: bridge_generated.dart
![Captura de pantalla 2023-11-06 a la(s) 01 45 02](https://github.com/Tristan-2021/rust_flutter_example/assets/79274889/23a1c67b-5628-43e4-9eaf-6bb58f4bd4c5)
lo que tiene que hacer es lo siguiente:
![Captura de pantalla 2023-11-06 a la(s) 01 46 08](https://github.com/Tristan-2021/rust_flutter_example/assets/79274889/3bca2adc-12b8-4590-a556-83b29defce75)
y con eso ya esta solucionado.
Ya podrá correr el codigo
> flutter run
## Codigo Rust
En nuestro caso el codigo de Rust lo que hace es generar y representar un fractal conocido como el "árbol de Barnsley" utilizando el sistema de funciones iteradas de Barnsley
## Notas
Aunque aquì obviamos varios pasos importantes, lo que hacemos es una muestra rápido de como poder llamar còdigo Rust desde Flutter o viceversa. Si desea configuar su propio proyecto puede guiarse de este tutorial:
- https://www.youtube.com/@drunisadev
- [Lab: Write your first Flutter app](https://docs.flutter.dev/get-started/codelab)
- [Cookbook: Useful Flutter samples](https://docs.flutter.dev/cookbook)

For help getting started with Flutter development, view the
[online documentation](https://docs.flutter.dev/), which offers tutorials,
samples, guidance on mobile development, and a full API reference.

import 'dart:ffi';
import 'dart:io' as io;

import 'package:flutter_rust_example/bridge_generated.dart';

export 'bridge_definitions.dart';
// Re-export the bridge so it is only necessary to import this file.
export 'bridge_generated.dart';

const _base = 'rustflutterteachdemo';
// const buildNAme = "librustflutterteachdemo.so";5
// final dyLib = DynamicLibrary.open(buildNAme);
// final api = RustflutterteachdemoImpl(dyLib);
// On MacOS, the dynamic library is not bundled with the binary,
// but rather directly **linked** against the binary.
final _dylib = io.Platform.isWindows ? '$_base.dll' : 'lib$_base.so';

final FlutterrustexampleImpl api = FlutterrustexampleImpl(
    io.Platform.isIOS || io.Platform.isMacOS
        ? DynamicLibrary.executable()
        : DynamicLibrary.open(_dylib));

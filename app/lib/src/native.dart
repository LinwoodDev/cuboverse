import 'native/ffi.io.dart' if (dart.library.html) 'native/ffi.web.dart' as ffi;
import 'native/bridge_definitions.dart';

export 'native/bridge_definitions.dart';

Native get api => ffi.api;

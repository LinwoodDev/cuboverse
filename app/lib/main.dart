import 'package:args/args.dart';
import 'package:cuboverse/pages/home/page.dart';
import 'package:cuboverse/pages/options/page.dart';
import 'package:dynamic_color/dynamic_color.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_localized_locales/flutter_localized_locales.dart';
import 'package:go_router/go_router.dart';
import 'package:flutter_web_plugins/url_strategy.dart';
import 'package:material_leap/l10n/leap_localizations.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:window_manager/window_manager.dart';

import 'cubits/settings.dart';

import 'setup.dart'
    if (dart.library.html) 'setup_web.dart'
    if (dart.library.io) 'setup_io.dart';
import 'theme.dart';

String? dataPath;

Future<void> main(List<String> args) async {
  WidgetsFlutterBinding.ensureInitialized();

  usePathUrlStrategy();
  final argParser = ArgParser();
  argParser.addOption('path', abbr: 'p');
  final result = argParser.parse(args);
  dataPath = result['path'];

  final prefs = await SharedPreferences.getInstance();
  final settingsCubit = SettingsCubit(prefs);

  await setup(settingsCubit);
  runApp(
    BlocProvider.value(
      value: settingsCubit,
      child: CuboverseApp(),
    ),
  );
}

class CuboverseApp extends StatelessWidget {
  CuboverseApp({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    if (kIsWeb) return _buildApp(null, null);
    return DynamicColorBuilder(
      builder: (lightDynamic, darkDynamic) =>
          _buildApp(lightDynamic, darkDynamic),
    );
  }

  Widget _buildApp(ColorScheme? lightDynamic, ColorScheme? darkDynamic) {
    final virtualWindowFrameBuilder = VirtualWindowFrameInit();
    return BlocBuilder<SettingsCubit, CuboverseSettings>(
        buildWhen: (previous, current) =>
            previous.design != current.design ||
            previous.themeMode != current.themeMode ||
            previous.locale != current.locale,
        builder: (context, state) => MaterialApp.router(
              debugShowCheckedModeBanner: false,
              routerConfig: _router,
              title: applicationName,
              theme: getThemeData(state.design, false, lightDynamic),
              darkTheme: getThemeData(state.design, true, darkDynamic),
              themeMode: state.themeMode,
              locale: state.locale.isEmpty ? null : Locale(state.locale),
              localizationsDelegates: const [
                LeapLocalizations.delegate,
                LocaleNamesLocalizationsDelegate(),
                AppLocalizations.delegate,
              ],
              supportedLocales: AppLocalizations.supportedLocales,
              builder: virtualWindowFrameBuilder,
            ));
  }

  final GoRouter _router = GoRouter(
    routes: <RouteBase>[
      GoRoute(
        path: '/',
        builder: (context, state) => const MyHomePage(title: "Cuboverse"),
        routes: [
          GoRoute(
            path: 'options',
            builder: (context, state) => const OptionsPage(),
          ),
        ],
      ),
    ],
  );
}

const flavor = String.fromEnvironment('flavor');
const isNightly =
    flavor == 'nightly' || flavor == 'dev' || flavor == 'development';
const shortApplicationName = isNightly ? 'Cuboverse Nightly' : 'Cuboverse';
const applicationName = 'Linwood $shortApplicationName';

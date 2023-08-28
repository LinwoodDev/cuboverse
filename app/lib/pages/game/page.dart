import 'package:cuboverse/main.dart';
import 'package:cuboverse/src/native.dart';
import 'package:cuboverse/widgets/window.dart';
import 'package:flame/game.dart';
import 'package:flutter/material.dart';

import '../../game/world.dart';
import '../options/page.dart';
import '../../overlays/pause.dart';

class GamePage extends StatefulWidget {
  const GamePage({super.key});

  @override
  State<GamePage> createState() => _GamePageState();
}

class _GamePageState extends State<GamePage> with WidgetsBindingObserver {
  late final Future<WorldManager> _manager;

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addObserver(this);

    _manager = api.createWorldManager();
  }

  @override
  void dispose() {
    super.dispose();

    WidgetsBinding.instance.removeObserver(this);
    _close();
  }

  Future<void> _close() => _manager.then((value) => value.close());

  @override
  Future<bool> didPopRoute() async {
    await _close();
    return false;
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: const WindowTitleBar(
        onlyShowOnDesktop: true,
        title: Text(applicationName),
      ),
      body: FutureBuilder<WorldManager>(
        future: _manager,
        builder: (context, snapshot) {
          if (snapshot.hasError) {
            return Text("Error: ${snapshot.error}");
          }
          if (!snapshot.hasData) {
            return const Center(child: CircularProgressIndicator());
          }
          final worldManager = snapshot.data!;
          return GameWidget<CuboverseWorld>(
            game: CuboverseWorld(worldManager),
            overlayBuilderMap: {
              "pause": (context, game) => PauseOverlay(game: game),
              "options": (context, game) => OptionsOverlay(game: game),
            },
          );
        },
      ),
    );
  }
}

import 'package:cuboverse/src/native.dart';
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

class _GamePageState extends State<GamePage> {
  late final Future<WorldManager> _manager;

  @override
  void initState() {
    super.initState();

    _manager = api.createWorldManager();
  }

  @override
  void dispose() {
    super.dispose();

    _manager.then((value) => value.close());
  }

  @override
  Widget build(BuildContext context) {
    return FutureBuilder<WorldManager>(
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
    );
  }
}

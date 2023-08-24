import 'package:cuboverse/src/native.dart';
import 'package:flame/game.dart';
import 'package:flutter/material.dart';

import '../../game/world.dart';
import '../../overlays/pause.dart';

class GamePage extends StatelessWidget {
  const GamePage({super.key});

  @override
  Widget build(BuildContext context) {
    return FutureBuilder<WorldManager>(
      future: api.createWorldManager(),
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
            "pause": (context, game) {
              return PauseOverlay(game: game);
            },
          },
        );
      },
    );
  }
}

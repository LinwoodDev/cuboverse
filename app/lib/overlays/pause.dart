import 'package:cuboverse/game/world.dart';
import 'package:flutter/material.dart';

class PauseOverlay extends StatelessWidget {
  final CuboverseWorld game;

  const PauseOverlay({super.key, required this.game});

  @override
  Widget build(BuildContext context) {
    return Container(
      color: Colors.black.withOpacity(0.5),
      child: Align(
        alignment: Alignment.bottomLeft,
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 200, maxHeight: 400),
          child: Material(
            shape: const RoundedRectangleBorder(
              borderRadius: BorderRadius.only(
                topRight: Radius.circular(16),
              ),
            ),
            clipBehavior: Clip.antiAlias,
            elevation: 20,
            type: MaterialType.canvas,
            child: ListView(
              shrinkWrap: true,
              children: [
                ListTile(
                  title: const Text("Resume"),
                  onTap: () {
                    game.resumeEngine();
                    game.overlays.remove("pause");
                  },
                ),
                ListTile(
                  title: const Text("Options"),
                  onTap: () {
                    game.overlays.add("options");
                  },
                ),
                ListTile(
                  title: const Text("Quit"),
                  onTap: () {
                    Navigator.of(context).pop();
                  },
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}

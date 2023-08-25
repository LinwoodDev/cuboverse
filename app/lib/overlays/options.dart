import 'package:cuboverse/game/world.dart';
import 'package:flutter/material.dart';
import 'package:material_leap/material_leap.dart';
import 'package:phosphor_flutter/phosphor_flutter.dart';

class OptionsOverlay extends StatelessWidget {
  final CuboverseWorld game;

  const OptionsOverlay({super.key, required this.game});

  @override
  Widget build(BuildContext context) {
    return Container(
      color: Colors.black.withOpacity(0.5),
      child: Align(
        alignment: Alignment.center,
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 500, maxHeight: 600),
          child: Material(
              clipBehavior: Clip.antiAlias,
              elevation: 20,
              type: MaterialType.card,
              borderRadius: BorderRadius.circular(16),
              child: DefaultTabController(
                length: 5,
                child: Column(
                  children: [
                    Header(
                      title: const Text("Options"),
                      actions: [
                        IconButton(
                          onPressed: () => game.overlays.remove("options"),
                          icon: const PhosphorIcon(PhosphorIconsLight.x),
                        ),
                      ],
                    ),
                    const TabBar(
                      isScrollable: true,
                      tabs: [
                        Tab(
                          text: "World",
                          icon: PhosphorIcon(
                              PhosphorIconsLight.globeHemisphereWest),
                        ),
                        Tab(
                          text: "Networking",
                          icon: PhosphorIcon(PhosphorIconsLight.cloud),
                        ),
                        Tab(
                          text: "Graphics",
                          icon: PhosphorIcon(PhosphorIconsLight.imageSquare),
                        ),
                        Tab(
                          text: "Personalization",
                          icon: PhosphorIcon(PhosphorIconsLight.faders),
                        ),
                        Tab(
                          text: "Data",
                          icon: PhosphorIcon(PhosphorIconsLight.database),
                        ),
                      ],
                    ),
                  ],
                ),
              )),
        ),
      ),
    );
  }
}

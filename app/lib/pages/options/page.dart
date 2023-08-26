import 'package:cuboverse/game/world.dart';
import 'package:cuboverse/widgets/window.dart';
import 'package:flutter/material.dart';
import 'package:material_leap/material_leap.dart';
import 'package:phosphor_flutter/phosphor_flutter.dart';

enum OptionsTab {
  world,
  networking,
  graphics,
  personalization,
  data,
  info;

  bool get isGameTab => this == OptionsTab.world;

  String getLocalizedName() => switch (this) {
        OptionsTab.world => "World",
        OptionsTab.networking => "Networking",
        OptionsTab.graphics => "Graphics",
        OptionsTab.personalization => "Personalization",
        OptionsTab.data => "Data",
        OptionsTab.info => "Info",
      };

  IconGetter get icon => switch (this) {
        OptionsTab.world => PhosphorIcons.globeHemisphereWest,
        OptionsTab.networking => PhosphorIcons.cloud,
        OptionsTab.graphics => PhosphorIcons.imageSquare,
        OptionsTab.personalization => PhosphorIcons.faders,
        OptionsTab.data => PhosphorIcons.database,
        OptionsTab.info => PhosphorIcons.info,
      };
}

class OptionsOverlay extends StatelessWidget {
  final CuboverseWorld game;

  const OptionsOverlay({super.key, required this.game});

  @override
  Widget build(BuildContext context) {
    const tabs = OptionsTab.values;
    return Container(
      color: Colors.black.withOpacity(0.5),
      child: Align(
        alignment: Alignment.center,
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 600, maxHeight: 600),
          child: Material(
              clipBehavior: Clip.antiAlias,
              elevation: 20,
              type: MaterialType.card,
              borderRadius: BorderRadius.circular(16),
              child: DefaultTabController(
                length: tabs.length,
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
                    _buildOptionsTabBar(tabs),
                    const Expanded(child: _OptionsTabBarView(tabs: tabs)),
                  ],
                ),
              )),
        ),
      ),
    );
  }
}

class OptionsPage extends StatelessWidget {
  const OptionsPage({super.key});

  @override
  Widget build(BuildContext context) {
    final tabs = OptionsTab.values.where((e) => !e.isGameTab).toList();
    return DefaultTabController(
      length: tabs.length,
      child: Scaffold(
        appBar: WindowTitleBar(
          title: const Text("Options"),
          bottom: _buildOptionsTabBar(tabs),
        ),
        body: _OptionsTabBarView(tabs: tabs),
      ),
    );
  }
}

TabBar _buildOptionsTabBar(List<OptionsTab> tabs) => TabBar(
      isScrollable: true,
      tabs: tabs
          .map((e) => Tab(
              text: e.getLocalizedName(),
              icon: PhosphorIcon(e.icon(PhosphorIconsStyle.light))))
          .toList(),
    );

class _OptionsTabBarView extends StatelessWidget {
  final List<OptionsTab> tabs;

  const _OptionsTabBarView({required this.tabs});

  @override
  Widget build(BuildContext context) => TabBarView(
        children: tabs
            .map((e) => switch (e) {
                  _ => Text(e.getLocalizedName()),
                })
            .toList(),
      );
}

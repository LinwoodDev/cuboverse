import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:phosphor_flutter/phosphor_flutter.dart';
import 'package:window_manager/window_manager.dart';

import '../cubits/settings.dart';

final isWindow =
    !kIsWeb && (Platform.isWindows || Platform.isLinux || Platform.isMacOS);

class WindowTitleBar extends StatelessWidget implements PreferredSizeWidget {
  final List<Widget> actions;
  final Widget? title;
  final Widget? leading;
  final PreferredSizeWidget? bottom;
  final bool onlyShowOnDesktop;
  final bool inView;
  final Color? backgroundColor;
  final double height;
  final double? leadingWidth;

  const WindowTitleBar({
    super.key,
    this.title,
    this.leading,
    this.bottom,
    this.leadingWidth,
    this.backgroundColor,
    this.actions = const [],
    this.onlyShowOnDesktop = false,
    this.inView = false,
    this.height = 70,
  });

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<SettingsCubit, CuboverseSettings>(
        buildWhen: (previous, current) =>
            previous.nativeTitleBar != current.nativeTitleBar,
        builder: (context, settings) {
          final isDesktop = isWindow && !kIsWeb;
          if (onlyShowOnDesktop && !isDesktop) return const SizedBox.shrink();
          return AppBar(
            title: title,
            backgroundColor: backgroundColor,
            automaticallyImplyLeading: !inView,
            leading: leading,
            bottom: bottom,
            leadingWidth: leadingWidth,
            toolbarHeight: height,
            flexibleSpace: const WindowFreeSpace(),
            actions: [
              ...actions,
              if (isDesktop && !inView)
                WindowButtons(
                  divider: actions.isNotEmpty,
                ),
            ],
          );
        });
  }

  @override
  Size get preferredSize =>
      Size.fromHeight(height + (bottom?.preferredSize.height ?? 0));
}

class WindowFreeSpace extends StatelessWidget {
  const WindowFreeSpace({super.key});

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<SettingsCubit, CuboverseSettings>(
        buildWhen: (previous, current) =>
            previous.nativeTitleBar != current.nativeTitleBar,
        builder: (context, settings) {
          if (!isWindow || kIsWeb || settings.nativeTitleBar) {
            return const SizedBox.shrink();
          }
          return GestureDetector(
            child: DragToMoveArea(
              child: Container(
                color: Colors.transparent,
              ),
            ),
            onSecondaryTap: () => windowManager.popUpWindowMenu(),
            onLongPress: () => windowManager.popUpWindowMenu(),
          );
        });
  }
}

class WindowButtons extends StatefulWidget {
  final bool divider;

  const WindowButtons({super.key, this.divider = true});

  @override
  State<WindowButtons> createState() => _WindowButtonsState();
}

class _WindowButtonsState extends State<WindowButtons> with WindowListener {
  bool maximized = false, alwaysOnTop = false;

  @override
  void initState() {
    if (!kIsWeb && isWindow) {
      windowManager.addListener(this);
    }
    super.initState();
    updateStates();
  }

  @override
  void dispose() {
    windowManager.removeListener(this);
    super.dispose();
  }

  Future<void> updateStates() async {
    final nextMaximized = await windowManager.isMaximized();
    final nextAlwaysOnTop = await windowManager.isAlwaysOnTop();
    if (mounted) {
      setState(() {
        maximized = nextMaximized;
        alwaysOnTop = nextAlwaysOnTop;
      });
    }
  }

  @override
  void onWindowUnmaximize() => setState(() => maximized = false);

  @override
  void onWindowMaximize() => setState(() => maximized = true);

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<SettingsCubit, CuboverseSettings>(
        buildWhen: (previous, current) =>
            previous.nativeTitleBar != current.nativeTitleBar,
        builder: (context, settings) {
          if (!kIsWeb && isWindow && !settings.nativeTitleBar) {
            return LayoutBuilder(
              builder: (context, constraints) => Card(
                child: Padding(
                  padding: const EdgeInsets.symmetric(vertical: 4.0),
                  child: ConstrainedBox(
                    constraints: const BoxConstraints(maxHeight: 42),
                    child: Row(
                      crossAxisAlignment: CrossAxisAlignment.center,
                      children: [
                        if (widget.divider) const VerticalDivider(),
                        Builder(builder: (context) {
                          return Row(
                            children: [
                              IconButton(
                                icon: const PhosphorIcon(
                                    PhosphorIconsLight.minus),
                                tooltip: AppLocalizations.of(context).minimize,
                                splashRadius: 20,
                                onPressed: () => windowManager.minimize(),
                              ),
                              TextButton(
                                child: Tooltip(
                                  message: maximized
                                      ? AppLocalizations.of(context).restore
                                      : AppLocalizations.of(context).maximize,
                                  child: PhosphorIcon(
                                    PhosphorIconsLight.square,
                                    size: maximized ? 14 : 20,
                                    color: Theme.of(context).iconTheme.color,
                                  ),
                                ),
                                onPressed: () async =>
                                    await windowManager.isMaximized()
                                        ? windowManager.unmaximize()
                                        : windowManager.maximize(),
                              ),
                              IconButton(
                                icon: const PhosphorIcon(PhosphorIconsLight.x),
                                tooltip: AppLocalizations.of(context).close,
                                color: Colors.red,
                                splashRadius: 20,
                                onPressed: () => windowManager.close(),
                              )
                            ]
                                .map((e) =>
                                    AspectRatio(aspectRatio: 1, child: e))
                                .toList(),
                          );
                        })
                      ],
                    ),
                  ),
                ),
              ),
            );
          }
          return Container();
        });
  }
}
